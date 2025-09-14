use alloy::{
    hex::FromHex,
    primitives::{Address, Bytes, U128, U256},
    providers::{ext::TxPoolApi, Provider},
    rpc::types::TransactionReceipt,
};
use anyhow::Result;
use chrono::Utc;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

use crate::{
    blockchain::{IUniversalRouter::IUniversalRouterInstance, NetworkConfigurator},
    models::profit_token::ProfitTokenRegistry,
    utils::{
        abi,
        config::{GasConfig, GasStrategy},
        encode_packed::SolidityDataType,
        metrics::{Metrics, OpportunityMetrics},
        utils::OpportunityStatus,
    },
};

use super::Opportunity;

const GAS_PRICE_INCREASE: u128 = 10000000000; // 10 gwei
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorGasConfig {
    /// Gas strategy
    pub gas_strategy: GasStrategy,
    /// Gas bid percentage on profit
    pub gas_bid_percentage_on_profit: Option<f64>,
    /// Minimum gas multiplier
    pub gas_multiplier: Option<f64>,
    /// Maximum gas in token
    pub max_gas_price: Option<u128>,
    /// Minimum gas price
    pub min_gas_price: Option<u128>,
    /// Gas limit for the transaction
    pub gas_limit: u64,
}
pub struct Executor<P: Provider + Send + Sync + 'static> {
    router_instances: Vec<IUniversalRouterInstance<Arc<P>>>,
    gas_config: ExecutorGasConfig,
    network_configurator: Arc<NetworkConfigurator<P>>,
    metrics: Arc<RwLock<Metrics>>,
    profit_token_registry: Arc<ProfitTokenRegistry>,
    provider: Arc<P>,
    logger_event_tx: Sender<(Opportunity, Result<TransactionReceipt>, OpportunityMetrics)>,
}

impl<P: Provider + Send + Sync + 'static> Executor<P> {
    pub fn new(
        providers: Vec<Arc<P>>,
        router_address: Address,
        gas_config: GasConfig,
        network_configurator: Arc<NetworkConfigurator<P>>,
        profit_token_registry: Arc<ProfitTokenRegistry>,
        metrics: Arc<RwLock<Metrics>>,
        logger_event_tx: Sender<(Opportunity, Result<TransactionReceipt>, OpportunityMetrics)>,
    ) -> Self {
        let router_instances = providers
            .iter()
            .map(|provider| IUniversalRouterInstance::new(router_address, provider.clone()))
            .collect();
        let max_gas_price = if let Some(max_gas_in_token) = gas_config.max_gas_in_token {
            Some(
                max_gas_in_token.parse::<u128>().unwrap()
                    / gas_config.gas_limit.unwrap_or(2_000_000) as u128,
            )
        } else {
            None
        };
        let min_gas_price = if let Some(min_gas_price) = gas_config.min_gas_price {
            Some(min_gas_price.parse::<u128>().unwrap())
        } else {
            None
        };
        Self {
            router_instances,
            gas_config: ExecutorGasConfig {
                gas_strategy: gas_config.gas_strategy,
                gas_bid_percentage_on_profit: gas_config.gas_bid_percentage_on_profit,
                gas_multiplier: gas_config.gas_multiplier,
                max_gas_price: max_gas_price,
                gas_limit: gas_config.gas_limit.unwrap_or(2_000_000),
                min_gas_price: min_gas_price,
            },
            network_configurator,
            metrics,
            profit_token_registry,
            provider: providers[0].clone(),
            logger_event_tx,
        }
    }

    pub async fn execute(&self, mut opportunity: Opportunity) -> Result<()> {
        match self.execute_tx(&mut opportunity).await {
            Ok(receipt) => {
                let opportunity_metrics = self
                    .metrics
                    .read()
                    .await
                    .get_opportunity_metrics_clone(
                        opportunity.transaction_hash,
                        opportunity.log_index,
                    )
                    .unwrap();
                self.logger_event_tx
                    .send((opportunity, Ok(receipt), opportunity_metrics))
                    .await?;
                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                let opportunity_metrics = self
                    .metrics
                    .read()
                    .await
                    .get_opportunity_metrics_clone(
                        opportunity.transaction_hash,
                        opportunity.log_index,
                    )
                    .unwrap();
                self.logger_event_tx
                    .send((
                        opportunity,
                        Err(anyhow::anyhow!(error_msg.clone())),
                        opportunity_metrics,
                    ))
                    .await?;
                Err(e)
            }
        }
    }

    async fn execute_tx(&self, opportunity: &mut Opportunity) -> Result<TransactionReceipt> {
        let reverse_cycle: Vec<_> = opportunity.cycle.iter().rev().cloned().collect();
        let path = reverse_cycle
            .iter()
            .flat_map(|p| {
                vec![
                    SolidityDataType::Address(p.token_in),
                    SolidityDataType::Address(p.pool),
                ]
            })
            .collect::<Vec<_>>();
        let (_, encoded) = abi::encode_packed(&path);
        let amount = opportunity.profit_token_amount + opportunity.profit;
        let data = Bytes::from_hex(encoded)?;

        let gas_config = self.gas_config.clone();
        let gas_limit = gas_config.gas_limit;

        let network_gas_price = self.network_configurator.get_gas_price().await?;
        let wrap_native = self.profit_token_registry.get_wrap_native().await;
        let network_gas_in_usd = self
            .profit_token_registry
            .get_value(
                &wrap_native,
                U256::from(gas_limit) * U256::from(network_gas_price),
            )
            .await
            .unwrap();

        let profit_plus_gas_in_usd = self
            .profit_token_registry
            .get_value(&opportunity.profit_token, opportunity.profit)
            .await
            .unwrap();
        if profit_plus_gas_in_usd < self.profit_token_registry.min_profit_usd + network_gas_in_usd {
            opportunity.status = OpportunityStatus::Skipped;
            return Err(anyhow::anyhow!(
                "Profit is less than min profit in usd + network gas in usd"
            ));
        }
        let max_profitable_gas_in_usd =
            profit_plus_gas_in_usd - self.profit_token_registry.min_profit_usd;

        let mut gas_price = self
            .get_gas_price(
                &gas_config,
                gas_config.gas_strategy.clone(),
                network_gas_price,
                gas_limit,
                wrap_native,
                network_gas_in_usd,
                profit_plus_gas_in_usd,
                max_profitable_gas_in_usd,
            )
            .await?;

        if let Some(min_gas_price) = gas_config.min_gas_price {
            if gas_price < min_gas_price {
                gas_price = min_gas_price;
            }
        }

        self.metrics.write().await.set_sent_at(
            opportunity.transaction_hash,
            opportunity.log_index,
            Utc::now().timestamp_millis() as u64,
        );

        let mut handles = Vec::new();
        for router_instance in &self.router_instances {
            let router = router_instance.clone();
            let data = data.clone();
            handles.push(tokio::spawn(async move {
                let mut tx = router.swap(amount, data);
                if gas_limit > 0 {
                    tx = tx.gas(gas_limit);
                }
                if gas_price > 0 {
                    tx = tx.gas_price(gas_price);
                }
                let tx = tx.send().await;

                tx
            }));
        }
        // Wait for first successful transaction
        let mut receipt = None;
        let mut errors = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(tx)) => {
                    if let Ok(r) = tx.get_receipt().await {
                        receipt = Some(r);
                        break;
                    }
                }
                Ok(Err(e)) => {
                    warn!("Failed to send transaction: {}", e);
                    errors.push(e);
                    continue;
                }
                Err(e) => {
                    warn!("Task failed: {}", e);
                    continue;
                }
            }
        }

        let receipt = receipt.ok_or_else(|| {
            opportunity.status = OpportunityStatus::Error;
            anyhow::anyhow!("All providers failed: {:?}", errors)
        })?;
        self.metrics.write().await.set_executed_at(
            opportunity.transaction_hash,
            opportunity.log_index,
            Utc::now().timestamp_millis() as u64,
        );

        info!("Executed tx: {:?}", receipt.transaction_hash);
        Ok(receipt)
    }

    async fn get_gas_price(
        &self,
        gas_config: &ExecutorGasConfig,
        strategy: GasStrategy,
        network_gas_price: u128,
        gas_limit: u64,
        wrap_native: Address,
        network_gas_in_usd: f64,
        profit_plus_gas_in_usd: f64,
        max_profitable_gas_in_usd: f64,
    ) -> Result<u128> {
        let mut gas_price;
        match strategy {
            GasStrategy::Multiplier => {
                gas_price = (network_gas_price as f64 * gas_config.gas_multiplier.unwrap()) as u128;
                let gas_in_token = U256::from(gas_limit) * U256::from(gas_price);
                let gas_in_usd = self
                    .profit_token_registry
                    .get_value(&wrap_native, gas_in_token)
                    .await
                    .unwrap();
                if gas_in_usd > max_profitable_gas_in_usd {
                    gas_price = (max_profitable_gas_in_usd / gas_in_usd) as u128;
                }

                if let Some(max_gas_price) = gas_config.max_gas_price {
                    if gas_price > max_gas_price {
                        gas_price = max_gas_price;
                    }
                }
            }
            GasStrategy::PercentageOfProfit => {
                let profit_in_usd = profit_plus_gas_in_usd - network_gas_in_usd;
                let gas_fee_in_usd = profit_in_usd
                    * gas_config.gas_bid_percentage_on_profit.unwrap()
                    + network_gas_in_usd;

                let gas_fee_in_token = self
                    .profit_token_registry
                    .get_amount_for_value(&wrap_native, gas_fee_in_usd)
                    .await
                    .unwrap();
                gas_price = gas_fee_in_token.to::<U128>().to::<u128>() / (gas_limit as u128);

                if let Some(max_gas_price) = gas_config.max_gas_price {
                    if gas_price > max_gas_price {
                        gas_price = max_gas_price;
                    }
                }
            }
            GasStrategy::Mempool => match self.provider.txpool_inspect().await {
                Ok(txpool_inspect) => {
                    let pending_txs = txpool_inspect.pending.values().flat_map(|txs| txs.values());
                    let highest_gas_price = pending_txs
                        .filter_map(|tx| Some(tx.gas_price))
                        .max()
                        .unwrap_or(network_gas_price);
                    gas_price = highest_gas_price + GAS_PRICE_INCREASE;
                    if let Some(max_gas_price) = gas_config.max_gas_price {
                        if gas_price > max_gas_price {
                            return Err(anyhow::anyhow!("Gas price is greater than max gas price"));
                        }
                    }

                    let gas_in_token = U256::from(gas_limit) * U256::from(gas_price);
                    let gas_in_usd = self
                        .profit_token_registry
                        .get_value(&wrap_native, gas_in_token)
                        .await
                        .unwrap();
                    if gas_in_usd > max_profitable_gas_in_usd {
                        return Err(anyhow::anyhow!(
                            "Gas price is greater than max profitable gas in usd"
                        ));
                    }
                }
                Err(e) => {
                    warn!("Failed to get gas price: {}", e);
                    return Box::pin(self.get_gas_price(
                        gas_config,
                        GasStrategy::PercentageOfProfit,
                        network_gas_price,
                        gas_limit,
                        wrap_native,
                        network_gas_in_usd,
                        profit_plus_gas_in_usd,
                        max_profitable_gas_in_usd,
                    ))
                    .await;
                }
            },
        }

        Ok(gas_price)
    }
}
