use crate::blockchain::{IVerioIP, IERC4626};
use crate::core::Database;
use crate::models::pool::base::{PoolTypeTrait, TopicList};
use crate::models::pool::erc4626::{ERC4626Pool, ERC4626Standard};
use crate::models::pool::EventApplicable;
use crate::models::token::TokenRegistry;
use crate::{PoolInterface, PoolType};
use alloy::eips::BlockId;
use alloy::providers::Provider;
use alloy::sol_types::SolEvent;
use alloy::{
    primitives::{Address, FixedBytes, U256},
    rpc::types::Log,
};
use anyhow::{anyhow, Result};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct VerioIP {
    base: ERC4626Standard,
}

impl VerioIP {
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
            base: ERC4626Standard::new(
                address,
                vault_token,
                asset_token,
                vault_reserve,
                asset_reserve,
                deposit_fee,
                withdraw_fee,
            ),
        }
    }

    pub fn save_to_db(&self, chain_id: u64, db: &Database) -> Result<()> {
        let key = self.base.address.to_string();
        db.insert(&format!("{}-verio_ip_pools", chain_id), key, self)?;
        debug!("Saved Verio IP pool {} to database", self.base.address);
        Ok(())
    }

    pub fn load_from_db(chain_id: u64, db: &Database, address: &Address) -> Result<Option<Self>> {
        let key = address.to_string();
        let pool = db.get::<_, Self>(&format!("{}-verio_ip_pools", chain_id), key)?;
        if let Some(ref _loaded_pool) = pool {
            debug!("Loaded Verio IP pool {} from database", address);
        }
        Ok(pool)
    }

    pub fn load_all_from_db(chain_id: u64, db: &Database) -> Result<Vec<Self>> {
        let mut pools = Vec::new();
        let iter = db.iter::<Self>(&format!("{}-verio_ip_pools", chain_id))?;

        for result in iter {
            match result {
                Ok((_, pool)) => pools.push(pool),
                Err(e) => error!("Error loading Verio IP pool: {}", e),
            }
        }

        info!("Loaded {} Verio IP pools from database", pools.len());
        Ok(pools)
    }
}

impl PoolInterface for VerioIP {
    fn address(&self) -> Address {
        self.base.address
    }

    /// Calculate output amount for a swap given an input amount and token
    fn calculate_output(&self, token_in: &Address, amount_in: U256) -> Result<U256> {
        self.base.calculate_output(token_in, amount_in)
    }

    /// Calculate input amount for a swap given an output amount and token
    fn calculate_input(&self, token_out: &Address, amount_out: U256) -> Result<U256> {
        self.base.calculate_input(token_out, amount_out)
    }

    /// Apply a swap to the pool state
    fn apply_swap(
        &mut self,
        _token_in: &Address,
        _amount_in: U256,
        _amount_out: U256,
    ) -> Result<()> {
        return Err(anyhow!("Not implemented"));
    }

    /// Get the tokens in the pool
    fn tokens(&self) -> (Address, Address) {
        self.base.tokens()
    }

    /// Get token0 of the pool
    fn token0(&self) -> Address {
        self.tokens().0
    }

    /// Get token1 of the pool
    fn token1(&self) -> Address {
        self.tokens().1
    }

    /// Get the pool fee as a fraction (e.g., 0.003 for 0.3%)
    fn fee(&self) -> f64 {
        self.base.fee()
    }

    /// Get a unique identifier for the pool
    fn id(&self) -> String {
        self.base.id()
    }

    /// Check if the pool contains a token
    fn contains_token(&self, token: &Address) -> bool {
        self.base.contains_token(token)
    }

    /// Clone the pool interface to a box
    fn clone_box(&self) -> Box<dyn PoolInterface + Send + Sync> {
        Box::new(self.clone())
    }

    /// Log summary of the pool
    fn log_summary(&self) -> String {
        self.base.log_summary()
    }

    /// Helper method for downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PoolTypeTrait for VerioIP {
    fn pool_type(&self) -> PoolType {
        PoolType::ERC4626(ERC4626Pool::VerioIP)
    }
}

impl EventApplicable for VerioIP {
    fn apply_log(&mut self, log: &Log) -> Result<()> {
        match log.topic0() {
            Some(&IERC4626::Deposit::SIGNATURE_HASH) => {
                let deposit_data: IERC4626::Deposit = log.log_decode()?.inner.data;
                self.base.vault_reserve += deposit_data.shares;
                self.base.asset_reserve += deposit_data.assets;
            }
            Some(&IERC4626::Withdraw::SIGNATURE_HASH) => {
                let withdraw_data: IERC4626::Withdraw = log.log_decode()?.inner.data;
                self.base.vault_reserve -= withdraw_data.shares;
                self.base.asset_reserve -= withdraw_data.assets;
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}

impl TopicList for VerioIP {
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

impl fmt::Display for VerioIP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Verio IP Pool {} - {} (reserves: {}, {})",
            self.base.vault_token,
            self.base.asset_token,
            self.base.vault_reserve,
            self.base.asset_reserve
        )
    }
}

pub async fn fetch_verio_ip_pool<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_address: Address,
    _block_number: BlockId,
    _token_registry: &Arc<RwLock<TokenRegistry>>,
) -> Result<VerioIP> {
    let vault_token = Address::from_str(&"0x5267F7eE069CEB3D8F1c760c215569b79d0685aD").unwrap();
    let asset_token = Address::ZERO;
    let _vault = IVerioIP::new(pool_address, &provider);
    let (vault_reserve, asset_reserve) = (U256::ZERO, U256::ZERO);
    let deposit_fee = 0;
    let withdraw_fee = 0;

    Ok(VerioIP::new(
        pool_address,
        vault_token,
        asset_token,
        vault_reserve,
        asset_reserve,
        deposit_fee,
        withdraw_fee,
    ))
}
