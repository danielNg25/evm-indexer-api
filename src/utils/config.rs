use crate::models::pool::base::PoolType;
use crate::models::profit_token::price_updater::base::PriceSourceType;
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Chain configurations
    pub chain_configs: Vec<ChainConfigs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GasStrategy {
    /// Multiplier
    Multiplier,
    /// Percentage of profit
    PercentageOfProfit,
    /// Mempool
    Mempool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasConfig {
    /// Gas strategy
    pub gas_strategy: GasStrategy,
    /// Gas bid percentage on profit
    pub gas_bid_percentage_on_profit: Option<f64>,
    /// Minimum gas multiplier
    pub gas_multiplier: Option<f64>,
    /// Maximum gas in token
    pub max_gas_in_token: Option<String>,
    /// Minimum gas price
    pub min_gas_price: Option<String>,
    /// Gas limit for the transaction
    pub gas_limit: Option<u64>,
}

/// Wallet configuration
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Address of the wallet
    pub wallet: PrivateKeySigner,
    /// Providers to use for the wallet (multiple RPCs for better transaction submission)
    pub send_rpc_urls: Vec<String>,
    /// Router address
    pub router_address: Address,
    /// Gas config
    pub gas_config: GasConfig,
    /// Use simple nonce management
    pub use_simple_nonce_management: bool,
}

/// Configuration for a profit token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitTokenConfig {
    /// Token address
    pub token: String,
    /// Price source
    pub price_source: Option<PriceSourceType>,
    /// default price
    pub default_price: f64,
}

/// Configuration for a pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Pool address
    pub address: String,
    /// Pool type (optional, defaults to UniswapV2 for backward compatibility)
    #[serde(default)]
    pub pool_type: PoolType,
}

impl PoolConfig {
    /// Create a pool config from a string (for backward compatibility)
    pub fn from_string(address: String) -> Self {
        Self {
            address,
            pool_type: PoolType::UniswapV2, // Default for backward compatibility
        }
    }
}

/// Strategy-specific configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainConfigs {
    pub rpc_urls: Vec<String>,
    pub websocket_urls: Vec<String>,
    // pub bootstrap_rpc_url: String,
    pub start_block: u64,
    pub max_blocks_per_batch: u64,
    pub wait_time_for_startup: u64,
    pub use_websocket: bool,
    // pub wrap_native: String,
    pub custom_multicall_address: Option<String>,
    // pub min_profit_usd: f64,
    // pub profit_tokens: Vec<ProfitTokenConfig>,
    pub pools: Vec<PoolConfig>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_path: Option<String>,
    pub load_snapshot_pool: Option<bool>,
}

/// Strategy-specific configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainConfigRaws {
    pub rpc_urls: Vec<String>,
    pub websocket_urls: Vec<String>,
    // pub bootstrap_rpc_url: String,
    pub start_block: u64,
    pub max_blocks_per_batch: u64,
    pub wait_time_for_startup: u64,
    pub use_websocket: bool,
    // pub wrap_native: String,
    pub custom_multicall_address: Option<String>,
    // pub min_profit_usd: f64,
    // pub profit_tokens: Vec<ProfitTokenConfig>,
    #[serde(default)]
    pub pool_addresses: Vec<String>,
    #[serde(default)]
    pub pools_with_type: Vec<PoolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub chains: Vec<ChainConfigRaws>,
    // Profit config
    // pub wrap_native: String,
    // pub profit_tokens: Vec<ProfitTokenConfig>,
    // pub min_profit_usd: Option<f64>,
    // pub gas_config: GasConfig,
    // Executor config
    // pub private_key: String,
    // pub router_address: String,
    // pub gas_limit: Option<u64>,
    pub db_path: Option<String>,
    // pub snapshot_interval: Option<u64>,
    pub load_snapshot_pool: Option<bool>,
    pub use_websocket: Option<bool>,
    // pub use_simple_nonce_management: Option<bool>,
    pub wait_time_for_startup: Option<u64>, // wait time for startup in milliseconds
}

impl ConfigFile {
    pub fn load() -> Result<Self> {
        let strategy_path = PathBuf::from("configs/config.toml");
        let config = fs::read_to_string(strategy_path)?;
        Ok(toml::from_str(&config)?)
    }
}
impl AppConfig {
    /// Load configuration from a file
    pub fn load() -> Result<Self> {
        let config = ConfigFile::load()?;

        let mut chain_configs = Vec::new();

        for chain in config.chains {
            // Merge pool configurations from both old and new formats
            let mut all_pools = Vec::new();

            // Add pools from new format (pools_with_type)
            all_pools.extend(chain.pools_with_type.clone());

            // Add pools from old format (pool_addresses) - these will be converted to PoolConfig with default type
            let old_format_pools: Vec<PoolConfig> = chain
                .pool_addresses
                .clone()
                .into_iter()
                .map(PoolConfig::from_string)
                .collect();
            all_pools.extend(old_format_pools);

            // Remove duplicate pools based on address (new format takes precedence)
            let mut unique_pools_map = std::collections::HashMap::new();
            all_pools.reverse(); // Reverse the order of the pools so pool in new format takes precedence
            for pool in all_pools {
                unique_pools_map.insert(pool.address.clone(), pool);
            }
            let unique_pools: Vec<PoolConfig> = unique_pools_map.into_values().collect();

            let chain_config = ChainConfigs {
                rpc_urls: chain.rpc_urls,
                websocket_urls: chain.websocket_urls,
                // bootstrap_rpc_url: chain.bootstrap_rpc_url,
                start_block: chain.start_block,
                max_blocks_per_batch: chain.max_blocks_per_batch,
                wait_time_for_startup: chain.wait_time_for_startup,
                use_websocket: chain.use_websocket,
                custom_multicall_address: chain.custom_multicall_address,
                pools: unique_pools,
            };
            chain_configs.push(chain_config);
        }

        Ok(Self {
            database: DatabaseConfig {
                db_path: config.db_path,
                load_snapshot_pool: config.load_snapshot_pool,
            },
            chain_configs,
        })
    }
}
