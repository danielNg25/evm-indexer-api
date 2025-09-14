use crate::{
    blockchain::{get_or_fetch_token, IUniswapV2Pair, IV2PairUint256},
    core::Database,
    models::pool::{
        base::{EventApplicable, PoolInterface, PoolTypeTrait, TopicList},
        v2::{default_factory_fee_by_chain_id, get_v2_factory_fee},
    },
    PoolType,
};
use alloy::{eips::BlockId, providers::Provider};
use std::{str::FromStr, sync::Arc};

use tokio::sync::RwLock;

use crate::models::token::TokenRegistry;
use alloy::sol_types::SolEvent;
use alloy::{
    primitives::{Address, FixedBytes, U256},
    rpc::types::Log,
};
use anyhow::{anyhow, Result};
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;
const FEE_DENOMINATOR: u128 = 1000000;

/// UniswapV2 Pool implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniswapV2Pool {
    /// Pool address
    pub address: Address,
    /// First token address in the pool
    pub token0: Address,
    /// Second token address in the pool
    pub token1: Address,
    /// Reserve of token0
    pub reserve0: U256,
    /// Reserve of token1
    pub reserve1: U256,
    /// Pool fee (e.g., 0.003 for 0.3%)
    pub fee: U256,
    /// Last update timestamp
    pub last_updated: u64,
    /// Creation timestamp or block
    pub created_at: u64,
}

impl UniswapV2Pool {
    /// Create a new V2 pool
    pub fn new(
        address: Address,
        token0: Address,
        token1: Address,
        reserve0: U256,
        reserve1: U256,
        fee: U256,
    ) -> Self {
        let current_time = chrono::Utc::now().timestamp() as u64;
        Self {
            address,
            token0,
            token1,
            reserve0,
            reserve1,
            fee,
            last_updated: current_time,
            created_at: current_time,
        }
    }

    /// Update pool reserves
    pub fn update_reserves(&mut self, reserve0: U256, reserve1: U256) -> Result<()> {
        self.reserve0 = reserve0;
        self.reserve1 = reserve1;
        self.last_updated = chrono::Utc::now().timestamp() as u64;
        Ok(())
    }

    /// Calculate the constant product k = x * y
    pub fn constant_product(&self) -> U256 {
        self.reserve0 * self.reserve1
    }

    /// Check if the pool is valid (has non-zero reserves)
    pub fn is_valid(&self) -> bool {
        !self.reserve0.is_zero() && !self.reserve1.is_zero()
    }

    /// Calculate the output amount for a swap (token0 -> token1)
    fn calculate_output_0_to_1(&self, amount_in: U256) -> Result<U256> {
        if amount_in.is_zero() {
            return Err(anyhow!("Input amount cannot be zero"));
        }

        if !self.is_valid() {
            return Err(anyhow!("Pool reserves are invalid"));
        }

        let amount_in_with_fee: alloy::primitives::Uint<256, 4> =
            amount_in.saturating_mul(U256::from(U256::from(FEE_DENOMINATOR) - (self.fee)));
        let numerator = amount_in_with_fee * self.reserve1;
        let denominator = self.reserve0 * U256::from(FEE_DENOMINATOR) + amount_in_with_fee;
        // Can't return more than all reserves
        let output = numerator / denominator;
        if output >= self.reserve1 {
            return Err(anyhow!("Insufficient liquidity for swap"));
        }

        Ok(output)
    }

    /// Calculate the output amount for a swap (token1 -> token0)
    fn calculate_output_1_to_0(&self, amount_in: U256) -> Result<U256> {
        if amount_in.is_zero() {
            return Err(anyhow!("Input amount cannot be zero"));
        }

        if !self.is_valid() {
            return Err(anyhow!("Pool reserves are invalid"));
        }

        let amount_in_with_fee =
            amount_in.saturating_mul(U256::from(U256::from(FEE_DENOMINATOR) - (self.fee)));
        let numerator = amount_in_with_fee * self.reserve0;
        let denominator = self.reserve1 * U256::from(FEE_DENOMINATOR) + amount_in_with_fee;

        // Can't return more than all reserves
        let output = numerator / denominator;
        if output >= self.reserve0 {
            return Err(anyhow!("Insufficient liquidity for swap"));
        }

        Ok(output)
    }

    fn calculate_input_0_to_1(&self, amount_out: U256) -> Result<U256> {
        if amount_out.is_zero() {
            return Err(anyhow!("Output amount cannot be zero"));
        }

        if !self.is_valid() {
            return Err(anyhow!("Pool reserves are invalid"));
        }

        if amount_out >= self.reserve1 {
            return Err(anyhow!("Insufficient liquidity for swap"));
        }

        let numerator = self.reserve0 * amount_out * U256::from(FEE_DENOMINATOR);
        let denominator = (self.reserve1 - amount_out) * (U256::from(FEE_DENOMINATOR) - self.fee);

        // Add 1 to round up
        let input = (numerator / denominator) + U256::from(1);

        Ok(input)
    }

    fn calculate_input_1_to_0(&self, amount_out: U256) -> Result<U256> {
        if amount_out.is_zero() {
            return Err(anyhow!("Output amount cannot be zero"));
        }

        if !self.is_valid() {
            return Err(anyhow!("Pool reserves are invalid"));
        }

        if amount_out >= self.reserve0 {
            return Err(anyhow!("Insufficient liquidity for swap"));
        }

        let numerator = self.reserve1 * amount_out * U256::from(FEE_DENOMINATOR);
        let denominator = (self.reserve0 - amount_out) * (U256::from(FEE_DENOMINATOR) - self.fee);

        // Add 1 to round up
        let input = (numerator / denominator) + U256::from(1);

        Ok(input)
    }

    /// Save pool data to database
    pub fn save_to_db(&self, chain_id: u64, db: &Database) -> Result<()> {
        let key = self.address.to_string();
        db.insert(&format!("{}-v2_pools", chain_id), key, self)?;
        debug!("Saved V2 pool {} to database", self.address);
        Ok(())
    }

    /// Load pool data from database
    pub fn load_from_db(chain_id: u64, db: &Database, address: &Address) -> Result<Option<Self>> {
        let key = address.to_string();
        let pool = db.get::<_, Self>(&format!("{}-v2_pools", chain_id), key)?;
        if let Some(ref _loaded_pool) = pool {
            debug!("Loaded V2 pool {} from database", address);
        }
        Ok(pool)
    }

    /// Load all V2 pools from database
    pub fn load_all_from_db(chain_id: u64, db: &Database) -> Result<Vec<Self>> {
        let mut pools = Vec::new();
        let iter = db.iter::<Self>(&format!("{}-v2_pools", chain_id))?;

        for result in iter {
            match result {
                Ok((_, pool)) => pools.push(pool),
                Err(e) => error!("Error loading V2 pool: {}", e),
            }
        }

        info!("Loaded {} V2 pools from database", pools.len());
        Ok(pools)
    }
}

impl PoolInterface for UniswapV2Pool {
    fn calculate_output(&self, token_in: &Address, amount_in: U256) -> Result<U256> {
        if token_in == &self.token0 {
            self.calculate_output_0_to_1(amount_in)
        } else if token_in == &self.token1 {
            self.calculate_output_1_to_0(amount_in)
        } else {
            Err(anyhow!("Token not in pool"))
        }
    }

    fn calculate_input(&self, token_out: &Address, amount_out: U256) -> Result<U256> {
        if token_out == &self.token0 {
            self.calculate_input_1_to_0(amount_out)
        } else if token_out == &self.token1 {
            self.calculate_input_0_to_1(amount_out)
        } else {
            Err(anyhow!("Token not in pool"))
        }
    }

    fn apply_swap(&mut self, token_in: &Address, amount_in: U256, amount_out: U256) -> Result<()> {
        if token_in == &self.token0 {
            // Token0 -> Token1 swap
            if amount_out >= self.reserve1 {
                return Err(anyhow!("Insufficient liquidity for swap"));
            }
            self.reserve0 += amount_in;
            self.reserve1 -= amount_out;
        } else if token_in == &self.token1 {
            // Token1 -> Token0 swap
            if amount_out >= self.reserve0 {
                return Err(anyhow!("Insufficient liquidity for swap"));
            }
            self.reserve1 += amount_in;
            self.reserve0 -= amount_out;
        } else {
            return Err(anyhow!("Token not in pool"));
        }

        self.last_updated = chrono::Utc::now().timestamp() as u64;
        Ok(())
    }

    fn address(&self) -> Address {
        self.address
    }

    fn tokens(&self) -> (Address, Address) {
        (self.token0, self.token1)
    }

    fn fee(&self) -> f64 {
        self.fee.to::<u128>() as f64 / FEE_DENOMINATOR as f64
    }

    fn id(&self) -> String {
        format!("v2-{}-{}-{}", self.address, self.token0, self.token1)
    }

    fn log_summary(&self) -> String {
        format!(
            "V2 Pool {} - {} <> {} (reserves: {}, {})",
            self.address, self.token0, self.token1, self.reserve0, self.reserve1
        )
    }

    fn contains_token(&self, token: &Address) -> bool {
        *token == self.token0 || *token == self.token1
    }

    fn clone_box(&self) -> Box<dyn PoolInterface + Send + Sync> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EventApplicable for UniswapV2Pool {
    fn apply_log(&mut self, log: &Log) -> Result<()> {
        match log.topic0() {
            Some(&IUniswapV2Pair::Sync::SIGNATURE_HASH) => {
                let sync_data: IUniswapV2Pair::Sync = log.log_decode()?.inner.data;
                debug!(
                    "Applying V2Sync event to pool {}: reserve0={}, reserve1={}",
                    self.address, sync_data.reserve0, sync_data.reserve1
                );
                self.update_reserves(
                    U256::from(sync_data.reserve0),
                    U256::from(sync_data.reserve1),
                )?;
                Ok(())
            }
            Some(&IV2PairUint256::Sync::SIGNATURE_HASH) => {
                let sync_data: IV2PairUint256::Sync = log.log_decode()?.inner.data;
                debug!(
                    "Applying V2Sync event to pool {}: reserve0={}, reserve1={}",
                    self.address, sync_data.reserve0, sync_data.reserve1
                );
                self.update_reserves(sync_data.reserve0, sync_data.reserve1)?;
                Ok(())
            }
            Some(&IUniswapV2Pair::Swap::SIGNATURE_HASH) => Ok(()),
            _ => {
                trace!("Ignoring unknown event for V2 pool");
                Ok(())
            }
        }
    }
}

impl TopicList for UniswapV2Pool {
    fn topics() -> Vec<FixedBytes<32>> {
        vec![
            IUniswapV2Pair::Swap::SIGNATURE_HASH,
            IUniswapV2Pair::Sync::SIGNATURE_HASH,
            IV2PairUint256::Sync::SIGNATURE_HASH,
        ]
    }

    fn profitable_topics() -> Vec<FixedBytes<32>> {
        vec![IUniswapV2Pair::Swap::SIGNATURE_HASH]
    }
}

impl fmt::Display for UniswapV2Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "V2 Pool {} - {} <> {} (reserves: {}, {})",
            self.address, self.token0, self.token1, self.reserve0, self.reserve1
        )
    }
}

impl PoolTypeTrait for UniswapV2Pool {
    fn pool_type(&self) -> PoolType {
        PoolType::UniswapV2
    }
}

/// Fetches pool data for a V2 pool
pub async fn fetch_v2_pool<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_address: Address,
    block_number: BlockId,
    token_registry: &Arc<RwLock<TokenRegistry>>,
    multicall_address: Address,
) -> Result<UniswapV2Pool> {
    let pair_instance = IUniswapV2Pair::new(pool_address, &provider);

    let (token0_address, token1_address, reserve0, reserve1, factory) = match provider
        .multicall()
        .address(multicall_address)
        .add(pair_instance.token0())
        .add(pair_instance.token1())
        .add(pair_instance.getReserves())
        .add(pair_instance.factory())
        .block(block_number)
        .aggregate()
        .await
    {
        Ok(results) => (
            results.0,
            results.1,
            U256::from(results.2._reserve0),
            U256::from(results.2._reserve1),
            results.3,
        ),
        Err(_) => {
            let pair_instance = IV2PairUint256::new(pool_address, &provider);
            let multicall = provider
                .multicall()
                .address(Address::from_str("0x0c9A8dB3B6C58bC02b8473167b0062b543F3ED7f").unwrap())
                .add(pair_instance.token0())
                .add(pair_instance.token1())
                .add(pair_instance.getReserves())
                .block(block_number)
                .aggregate()
                .await?;
            (
                multicall.0,
                multicall.1,
                multicall.2._reserve0,
                multicall.2._reserve1,
                Address::ZERO,
            )
        }
    };
    // Create token objects (you'll need to fetch token details)
    let token0 =
        get_or_fetch_token(token_registry, provider, token0_address, multicall_address).await?;
    let token1 =
        get_or_fetch_token(token_registry, provider, token1_address, multicall_address).await?;

    let fee = match get_v2_factory_fee(&factory) {
        Ok(fee) => fee,
        Err(_) => {
            info!("Failed to get factory fee, using default");
            default_factory_fee_by_chain_id(provider.get_chain_id().await?, &factory)?
        }
    };

    // Create and return V2 pool
    Ok(UniswapV2Pool::new(
        pool_address,
        token0,
        token1,
        reserve0,
        reserve1,
        fee,
    ))
}
