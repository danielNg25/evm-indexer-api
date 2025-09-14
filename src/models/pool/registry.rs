use crate::core::Database;
use crate::models::pool::base::{PoolInterface, PoolType, Topic};
use crate::models::pool::erc4626::{ERC4626Pool, VerioIP};
use crate::models::pool::v2::UniswapV2Pool;
use crate::models::pool::v3::UniswapV3Pool;
use alloy::primitives::Address;
use anyhow::Result;
use log::info;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct PoolRegistry {
    by_address: Arc<RwLock<HashMap<Address, Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>>>>,
    by_type: Arc<RwLock<HashMap<PoolType, Vec<Address>>>>,
    last_processed_block: Arc<RwLock<u64>>,
    topics: Arc<RwLock<Vec<Topic>>>,
    profitable_topics: Arc<RwLock<HashSet<Topic>>>,
    network_id: u64,
}

impl PoolRegistry {
    pub fn new(network_id: u64) -> Self {
        Self {
            by_address: Arc::new(RwLock::new(HashMap::new())),
            by_type: Arc::new(RwLock::new(HashMap::new())),
            last_processed_block: Arc::new(RwLock::new(0)),
            topics: Arc::new(RwLock::new(Vec::new())),
            profitable_topics: Arc::new(RwLock::new(HashSet::new())),
            network_id,
        }
    }

    /// Set network ID for this registry
    pub fn set_network_id(&mut self, network_id: u64) {
        self.network_id = network_id;
    }

    /// Get network ID
    pub fn get_network_id(&self) -> u64 {
        self.network_id
    }

    /// Get total pool count
    pub async fn pool_count(&self) -> usize {
        self.by_address.read().await.len()
    }

    pub async fn add_pool(&self, pool: Box<dyn PoolInterface + Send + Sync>) {
        let address = pool.address();
        let pool_type = pool.pool_type();

        // Add to address map
        let mut address_map = self.by_address.write().await;
        address_map.insert(address, Arc::new(RwLock::new(pool)));

        // Add to type map
        let mut type_map = self.by_type.write().await;
        type_map
            .entry(pool_type)
            .or_insert_with(Vec::new)
            .push(address);
    }

    pub async fn get_pool(
        &self,
        address: &Address,
    ) -> Option<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let pools = self.by_address.read().await;
        pools.get(address).map(Arc::clone)
    }

    pub async fn remove_pool(
        &self,
        address: Address,
    ) -> Option<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        // Remove from address map
        let mut address_map = self.by_address.write().await;
        let pool = address_map.remove(&address)?;
        let pool_type = pool.read().await.pool_type();

        // Remove from type map
        let mut type_map = self.by_type.write().await;
        if let Some(addresses) = type_map.get_mut(&pool_type) {
            addresses.retain(|&a| a != address);
            if addresses.is_empty() {
                type_map.remove(&pool_type);
            }
        }

        Some(pool)
    }

    pub async fn get_all_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let pools = self.by_address.read().await;
        pools.values().map(Arc::clone).collect()
    }

    pub async fn get_pools_by_type(
        &self,
        pool_type: PoolType,
    ) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&pool_type)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_v2_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&PoolType::UniswapV2)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_v3_pools(&self) -> Vec<Arc<RwLock<Box<dyn PoolInterface + Send + Sync>>>> {
        let type_map = self.by_type.read().await;
        let address_map = self.by_address.read().await;

        type_map
            .get(&PoolType::UniswapV3)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| address_map.get(addr).map(Arc::clone))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn get_addresses_by_type(&self, pool_type: PoolType) -> Vec<Address> {
        let type_map = self.by_type.read().await;
        type_map
            .get(&pool_type)
            .map(|addresses| addresses.clone())
            .unwrap_or_default()
    }

    pub async fn get_v2_addresses(&self) -> Vec<Address> {
        self.get_addresses_by_type(PoolType::UniswapV2).await
    }

    pub async fn get_v3_addresses(&self) -> Vec<Address> {
        self.get_addresses_by_type(PoolType::UniswapV3).await
    }

    pub async fn get_all_addresses(&self) -> Vec<Address> {
        self.by_address.read().await.keys().cloned().collect()
    }

    pub async fn log_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Pool Registry Summary:\n");
        summary.push_str("--------------------------------\n");

        let pools = self.by_address.read().await;
        for (_, pool) in &*pools {
            summary.push_str(&format!("Pool: {}\n", pool.read().await.log_summary()));
        }
        summary
    }

    /// Save all pools to database
    pub async fn save_to_db(&self, db: &Database) -> Result<()> {
        let pools = self.by_address.read().await;
        let mut v2_count = 0;
        let mut v3_count = 0;

        for (_, pool_arc) in pools.iter() {
            let pool = pool_arc.read().await;

            // Determine pool type and save accordingly
            match pool.pool_type() {
                PoolType::UniswapV2 => {
                    if let Some(v2_pool) = pool.downcast_ref::<UniswapV2Pool>() {
                        v2_pool.save_to_db(self.network_id, db)?;
                        v2_count += 1;
                    }
                }
                PoolType::UniswapV3 => {
                    if let Some(v3_pool) = pool.downcast_ref::<UniswapV3Pool>() {
                        v3_pool.save_to_db(self.network_id, db)?;
                        v3_count += 1;
                    }
                }
                PoolType::ERC4626(ERC4626Pool::VerioIP) => {
                    if let Some(verio_ip_pool) = pool.downcast_ref::<VerioIP>() {
                        verio_ip_pool.save_to_db(self.network_id, db)?;
                    }
                }
            }
        }

        // Save the last processed block
        let last_block = self.get_last_processed_block().await;
        db.insert("metadata", "last_processed_block", &last_block)?;

        // Save topics
        db.insert("metadata", "topics", &self.get_topics().await)?;
        db.insert(
            "metadata",
            "profitable_topics",
            &self.get_profitable_topics().await,
        )?;

        // Final database snapshot to ensure everything is flushed
        db.snapshot()?;

        info!(
            "CHAIN ID: {} Saved {} V2 pools, {} V3 pools, and last processed block {} to database",
            self.network_id, v2_count, v3_count, last_block
        );
        Ok(())
    }

    /// Load pools from database
    pub async fn load_from_db(&self, db: &Database) -> Result<()> {
        // Load V2 pools
        let v2_pools = UniswapV2Pool::load_all_from_db(self.network_id, db)?;
        for pool in v2_pools {
            let boxed_pool: Box<dyn PoolInterface + Send + Sync> = Box::new(pool);
            self.add_pool(boxed_pool).await;
        }

        // Load V3 pools
        let v3_pools = UniswapV3Pool::load_all_from_db(self.network_id, db)?;
        for pool in v3_pools {
            let boxed_pool: Box<dyn PoolInterface + Send + Sync> = Box::new(pool);
            self.add_pool(boxed_pool).await;
        }

        // Load topics
        if let Ok(Some(topics)) = db.get::<_, Vec<Topic>>("metadata", "topics") {
            self.add_topics(topics).await;
        }

        // Load profitable topics
        if let Ok(Some(profitable_topics)) =
            db.get::<_, Vec<Topic>>("metadata", "profitable_topics")
        {
            self.add_profitable_topics(profitable_topics).await;
        }

        // Load the last processed block
        if let Ok(Some(last_block)) = db.get::<_, u64>("metadata", "last_processed_block") {
            self.set_last_processed_block(last_block).await;
            info!("Loaded last processed block: {}", last_block);
        } else {
            info!("No last processed block found in database");
        }

        let total_pools = self.by_address.read().await.len();
        info!("Loaded {} pools from database", total_pools);
        Ok(())
    }

    // Get the last processed block
    pub async fn get_last_processed_block(&self) -> u64 {
        *self.last_processed_block.read().await
    }

    // Set the last processed block
    pub async fn set_last_processed_block(&self, block_number: u64) {
        let mut block = self.last_processed_block.write().await;
        *block = block_number;
    }

    pub async fn add_topics(&self, topics: Vec<Topic>) {
        let mut topics_lock = self.topics.write().await;
        topics_lock.extend(topics);
    }

    pub async fn add_profitable_topics(&self, topics: Vec<Topic>) {
        let mut topics_lock = self.profitable_topics.write().await;
        topics_lock.extend(topics);
    }

    pub async fn get_topics(&self) -> Vec<Topic> {
        self.topics.read().await.clone()
    }

    pub async fn get_profitable_topics(&self) -> HashSet<Topic> {
        self.profitable_topics.read().await.clone()
    }
}

impl Clone for PoolRegistry {
    fn clone(&self) -> Self {
        Self {
            by_address: Arc::clone(&self.by_address),
            by_type: Arc::clone(&self.by_type),
            last_processed_block: Arc::clone(&self.last_processed_block),
            topics: Arc::clone(&self.topics),
            profitable_topics: Arc::clone(&self.profitable_topics),
            network_id: self.network_id.clone(),
        }
    }
}
