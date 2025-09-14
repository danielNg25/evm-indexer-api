use crate::blockchain::IERC4626;
use crate::models::pool::base::TopicList;
use crate::models::pool::erc4626::verio_ip::fetch_verio_ip_pool;
use crate::models::pool::erc4626::ERC4626Pool;
use crate::models::pool::EventApplicable;
use crate::PoolInterface;
use alloy::eips::BlockId;
use alloy::sol_types::SolEvent;
use alloy::{
    primitives::{Address, FixedBytes, U256},
    rpc::types::Log,
};
use anyhow::{anyhow, Result};

use alloy::providers::Provider;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::token::TokenRegistry;

const FEE_DENOMINATOR: u128 = 1000000;
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ERC4626Standard {
    /// Pool address
    pub address: Address,
    /// Token received from depositing, i.e. shares token
    pub vault_token: Address,
    /// Token received from withdrawing, i.e. underlying token
    pub asset_token: Address,
    /// Total supply of vault tokens
    pub vault_reserve: U256,
    /// Total balance of asset tokens held by vault
    pub asset_reserve: U256,
    /// Deposit fee in basis points
    pub deposit_fee: u32,
    /// Withdrawal fee in basis points
    pub withdraw_fee: u32,
}

impl ERC4626Standard {
    pub fn new(
        address: Address,
        vault_token: Address,
        asset_token: Address,
        vault_reserve: U256,
        asset_reserve: U256,
        deposit_fee: u32,
        withdraw_fee: u32,
    ) -> Self {
        Self {
            address,
            vault_token,
            asset_token,
            vault_reserve,
            asset_reserve,
            deposit_fee,
            withdraw_fee,
        }
    }
}

impl ERC4626Standard {
    pub fn address(&self) -> Address {
        self.address
    }

    /// Calculate output amount for a swap given an input amount and token
    pub fn calculate_output(&self, token_in: &Address, amount_in: U256) -> Result<U256> {
        if amount_in.is_zero() {
            return Ok(U256::ZERO);
        }

        if self.vault_reserve.is_zero() {
            return Ok(amount_in);
        }

        let (fee, reserve_in, reserve_out) = if token_in.eq(&self.vault_token) {
            (self.withdraw_fee, self.vault_reserve, self.asset_reserve)
        } else {
            (self.deposit_fee, self.asset_reserve, self.vault_reserve)
        };

        Ok(
            amount_in * reserve_out / reserve_in * U256::from(FEE_DENOMINATOR - fee as u128)
                / U256::from(FEE_DENOMINATOR),
        )
    }

    /// Calculate input amount for a swap given an output amount and token
    pub fn calculate_input(&self, token_out: &Address, amount_out: U256) -> Result<U256> {
        if amount_out.is_zero() {
            return Ok(U256::ZERO);
        }

        if self.asset_reserve.is_zero() {
            return Ok(amount_out);
        }

        let (fee, reserve_in, reserve_out) = if token_out.eq(&self.vault_token) {
            (self.withdraw_fee, self.vault_reserve, self.asset_reserve)
        } else {
            (self.deposit_fee, self.asset_reserve, self.vault_reserve)
        };

        Ok(
            amount_out * reserve_in / reserve_out * U256::from(FEE_DENOMINATOR - fee as u128)
                / U256::from(FEE_DENOMINATOR),
        )
    }

    /// Apply a swap to the pool state
    pub fn apply_swap(
        &mut self,
        _token_in: &Address,
        _amount_in: U256,
        _amount_out: U256,
    ) -> Result<()> {
        return Err(anyhow!("Not implemented"));
    }

    /// Get the tokens in the pool
    pub fn tokens(&self) -> (Address, Address) {
        (self.vault_token, self.asset_token)
    }

    /// Get token0 of the pool
    pub fn token0(&self) -> Address {
        self.tokens().0
    }

    /// Get token1 of the pool
    pub fn token1(&self) -> Address {
        self.tokens().1
    }

    /// Get the pool fee as a fraction (e.g., 0.003 for 0.3%)
    pub fn fee(&self) -> f64 {
        self.deposit_fee as f64 / FEE_DENOMINATOR as f64
    }

    /// Get a unique identifier for the pool
    pub fn id(&self) -> String {
        self.address.to_string()
    }

    /// Check if the pool contains a token
    pub fn contains_token(&self, token: &Address) -> bool {
        *token == self.vault_token || *token == self.asset_token
    }

    /// Clone the pool interface to a box
    // fn clone_box(&self) -> Box<dyn PoolInterface + Send + Sync> {
    //     Box::new(self.clone())
    // }

    /// Log summary of the pool
    pub fn log_summary(&self) -> String {
        format!(
            "ERC4626 Standard Pool {} - {} (reserves: {}, {})",
            self.vault_token, self.asset_token, self.vault_reserve, self.asset_reserve
        )
    }

    /// Helper method for downcasting
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EventApplicable for ERC4626Standard {
    fn apply_log(&mut self, log: &Log) -> Result<()> {
        match log.topic0() {
            Some(&IERC4626::Deposit::SIGNATURE_HASH) => {
                let deposit_data: IERC4626::Deposit = log.log_decode()?.inner.data;
                self.vault_reserve += deposit_data.shares;
                self.asset_reserve += deposit_data.assets;
            }
            Some(&IERC4626::Withdraw::SIGNATURE_HASH) => {
                let withdraw_data: IERC4626::Withdraw = log.log_decode()?.inner.data;
                self.vault_reserve -= withdraw_data.shares;
                self.asset_reserve -= withdraw_data.assets;
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}

impl TopicList for ERC4626Standard {
    fn topics() -> Vec<FixedBytes<32>> {
        vec![
            IERC4626::Deposit::SIGNATURE_HASH,
            IERC4626::Withdraw::SIGNATURE_HASH,
        ]
    }

    fn profitable_topics() -> Vec<FixedBytes<32>> {
        vec![
            IERC4626::Deposit::SIGNATURE_HASH,
            IERC4626::Withdraw::SIGNATURE_HASH,
        ]
    }
}

impl fmt::Display for ERC4626Standard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ERC4626 Standard Pool {} - {} (reserves: {}, {})",
            self.vault_token, self.asset_token, self.vault_reserve, self.asset_reserve
        )
    }
}

pub async fn fetch_erc4626_pool<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_type: ERC4626Pool,
    pool_address: Address,
    block_number: BlockId,
    token_registry: &Arc<RwLock<TokenRegistry>>,
) -> Result<Box<dyn PoolInterface>> {
    match pool_type {
        ERC4626Pool::VerioIP => {
            let pool =
                fetch_verio_ip_pool(provider, pool_address, block_number, token_registry).await?;
            Ok(Box::new(pool))
        }
    }
}
