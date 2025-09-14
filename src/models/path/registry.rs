use crate::core::Database;
use crate::models::pool::base::PoolInterface;
use crate::models::profit_token::ProfitTokenRegistry;
use alloy::primitives::Address;
use anyhow::Result;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTokenPair {
    pub pool: Address,
    pub token_in: Address,
    pub token_out: Address,
}

impl PoolTokenPair {
    pub fn to_string(&self) -> String {
        let pool_str = format!("{:?}", self.pool);
        let token_in_str = format!("{:?}", self.token_in);
        let token_out_str = format!("{:?}", self.token_out);

        format!(
            "Pool: {}...{} | Token In: {}...{} → Token Out: {}...{}",
            &pool_str[0..6],
            &pool_str[pool_str.len() - 4..],
            &token_in_str[0..6],
            &token_in_str[token_in_str.len() - 4..],
            &token_out_str[0..6],
            &token_out_str[token_out_str.len() - 4..]
        )
    }
}

/// Helper function to format a path of PoolTokenPair for nice console output
pub fn format_path(path: &[PoolTokenPair]) -> String {
    if path.is_empty() {
        return "[Empty Path]".to_string();
    }

    let mut result = String::from("");

    // If it's a cycle, add summary
    if !path.is_empty()
        && path.last().unwrap().token_out == path.first().unwrap().token_in
        && path.len() > 1
    {
        let first_token_str = format!("{:?}", path.first().unwrap().token_in);
        let first_token_short = first_token_str[0..10].to_string();

        let middle_tokens: String = path
            .iter()
            .map(|p| {
                let token_str = format!("{:?}", p.token_out);
                token_str[0..10].to_string()
            })
            .collect::<Vec<String>>()
            .join(" → ");

        result.push_str(&format!(
            "CYCLE DETECTED: {} → {}\n",
            first_token_short, middle_tokens
        ));
    }

    result
}

/// Helper function to format a cycle in a concise way
pub fn format_cycle_summary(cycle: &[PoolTokenPair]) -> String {
    if cycle.is_empty() {
        return "[Empty Cycle]".to_string();
    }

    // First token info
    let first_token_str = format!("{:?}", cycle.first().unwrap().token_in);
    let first_token_short = format!(
        "{}..{}",
        &first_token_str[0..6],
        &first_token_str[first_token_str.len() - 4..]
    );

    // Build formatted path showing tokens connected by pools
    let mut formatted_path = vec![first_token_short];

    for pair in cycle.iter() {
        // Add pool info
        let pool_str = format!("{:?}", pair.pool);
        let pool_short = format!("{}..{}", &pool_str[0..6], &pool_str[pool_str.len() - 4..]);
        formatted_path.push(format!("({})", pool_short));

        // Add token out
        let token_str = format!("{:?}", pair.token_out);
        let token_short = format!(
            "{}..{}",
            &token_str[0..6],
            &token_str[token_str.len() - 4..]
        );
        formatted_path.push(token_short);
    }

    // Join with arrows
    formatted_path.join(" → ")
}

#[derive(Default)]
pub struct PathRegistry {
    // Map from token address to pools that contain it
    token_to_pools: Arc<RwLock<HashMap<Address, HashSet<Address>>>>,
    // Map from pool address to its tokens
    pool_to_tokens: Arc<RwLock<HashMap<Address, HashSet<Address>>>>,
    // Cache of cycles found for each pool and start token
    // The Vec includes rotated cycles, original cycles, and rotation index: [(rotated_cycle, original_cycle, rotation_index)]
    cycles_cache: Arc<
        RwLock<HashMap<(Address, Address), Vec<(Vec<PoolTokenPair>, Vec<PoolTokenPair>, usize)>>>,
    >,
    // Reference to the global profit token registry
    profit_token_registry: Arc<ProfitTokenRegistry>,
    // Maximum length of paths to search for
    max_path_length: usize,
}

impl PathRegistry {
    pub fn new(profit_token_registry: Arc<ProfitTokenRegistry>, max_path_length: usize) -> Self {
        Self {
            token_to_pools: Arc::new(RwLock::new(HashMap::new())),
            pool_to_tokens: Arc::new(RwLock::new(HashMap::new())),
            cycles_cache: Arc::new(RwLock::new(HashMap::new())),
            profit_token_registry,
            max_path_length,
        }
    }

    pub async fn add_pool(&self, pool: &dyn PoolInterface) {
        let (token0, token1) = pool.tokens();
        let pool_address = pool.address();

        self.add_pool_by_address(pool_address, token0, token1).await;
    }

    pub async fn add_pool_by_address(
        &self,
        pool_address: Address,
        token0: Address,
        token1: Address,
    ) {
        // Update token_to_pools
        let mut token_map = self.token_to_pools.write().await;
        token_map.entry(token0).or_default().insert(pool_address);
        token_map.entry(token1).or_default().insert(pool_address);

        // Update pool_to_tokens
        let mut pool_map = self.pool_to_tokens.write().await;
        let tokens = pool_map.entry(pool_address).or_default();
        tokens.insert(token0);
        tokens.insert(token1);
    }

    pub async fn remove_pool(&self, pool_address: Address) {
        // Get pool's tokens
        let pool_tokens = {
            let pool_map = self.pool_to_tokens.read().await;
            pool_map.get(&pool_address).cloned().unwrap_or_default()
        };

        // Remove from token_to_pools
        let mut token_map = self.token_to_pools.write().await;
        for token in &pool_tokens {
            if let Some(pools) = token_map.get_mut(token) {
                pools.remove(&pool_address);
                if pools.is_empty() {
                    token_map.remove(token);
                }
            }
        }

        // Remove from pool_to_tokens
        let mut pool_map = self.pool_to_tokens.write().await;
        pool_map.remove(&pool_address);

        // Remove from cycles cache
        let mut cycles_cache = self.cycles_cache.write().await;
        cycles_cache.retain(|(pool, _), _| *pool != pool_address);
    }

    pub async fn find_all_cycles(&self) {
        // Clear existing cycles
        let mut cycles_cache = self.cycles_cache.write().await;
        cycles_cache.clear();
        drop(cycles_cache);

        // Get all pools
        let pools: Vec<Address> = {
            let pool_map = self.pool_to_tokens.read().await;
            pool_map.keys().cloned().collect()
        };

        // Find cycles for each pool and each start token
        for pool in pools {
            // Get pool's tokens
            let (token0, token1) = {
                let pool_map = self.pool_to_tokens.read().await;
                let tokens: Vec<Address> = pool_map.get(&pool).unwrap().iter().cloned().collect();
                (tokens[0], tokens[1])
            };

            // Find cycles for both start tokens
            self.find_cycles_for_start_token(pool, token0).await;
            self.find_cycles_for_start_token(pool, token1).await;
        }
    }

    async fn find_cycles_for_start_token(&self, start_pool: Address, start_token: Address) {
        info!(
            "Finding cycles for start pool: {} and start token: {}",
            start_pool, start_token
        );
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        // Get the other token in the pool
        let other_token = {
            let pool_map = self.pool_to_tokens.read().await;
            let tokens = pool_map.get(&start_pool).unwrap();
            tokens.iter().find(|&&t| t != start_token).cloned().unwrap()
        };
        self.dfs_find_cycles(
            start_pool,
            start_token,
            other_token,
            &mut visited,
            &mut path,
            &mut cycles,
        )
        .await;

        self.cycles_cache
            .write()
            .await
            .insert((start_pool, start_token), cycles);
        // drop(cycles_cache);
    }

    async fn dfs_find_cycles(
        &self,
        current_pool: Address,
        token_in: Address,
        token_out: Address,
        visited: &mut HashSet<Address>,
        path: &mut Vec<PoolTokenPair>,
        cycles: &mut Vec<(Vec<PoolTokenPair>, Vec<PoolTokenPair>, usize)>,
    ) {
        // If path length exceeds maximum, stop searching
        if path.len() >= self.max_path_length {
            return;
        }

        // If we've found a cycle back to the start pool
        if !path.is_empty() && current_pool == path[0].pool {
            // Check if the last token_out matches the first token_in
            if path.last().unwrap().token_out == path.first().unwrap().token_in {
                // Clone the current path
                let original_cycle = path.clone();

                // If the cycle contains a profit token, rotate it to start from the first profit token
                let mut has_profit_token = false;
                for pair in &original_cycle {
                    if self
                        .profit_token_registry
                        .is_profit_token(&pair.token_in)
                        .await
                    {
                        has_profit_token = true;
                        break;
                    }
                }

                if has_profit_token {
                    // Get the rotated cycle but keep the original unrotated cycle as well
                    if let Some((rotated_cycle, rotation_index)) =
                        self.rotate_cycle_to_profit_token(&original_cycle).await
                    {
                        info!("Found cycle: {}", format_path(&rotated_cycle));
                        cycles.push((rotated_cycle, original_cycle, rotation_index));
                    }
                } else {
                    info!("Cannot rotate cycle: {}", format_path(&original_cycle));
                    PathRegistry::write_cycle_to_file(&original_cycle, "invalid_cycles.txt")
                        .await
                        .unwrap();
                }
            }
            return;
        }

        // Skip if we've already visited this pool (except for the start pool)
        if !path.is_empty() && visited.contains(&current_pool) {
            return;
        }

        // Mark as visited (except for the start pool)
        if !path.is_empty() {
            visited.insert(current_pool);
        }

        // Add current step to path
        path.push(PoolTokenPair {
            pool: current_pool,
            token_in,
            token_out,
        });

        // Get all pools that share the output token
        let connected_pools = {
            let token_map = self.token_to_pools.read().await;
            token_map.get(&token_out).cloned().unwrap_or_default()
        };

        // Visit each connected pool
        for next_pool in connected_pools {
            if next_pool != current_pool {
                // Get the other token in the next pool
                let next_tokens = {
                    let pool_map = self.pool_to_tokens.read().await;
                    pool_map.get(&next_pool).cloned().unwrap_or_default()
                };

                // The next pool must use token_out as token_in
                if next_tokens.contains(&token_out) {
                    let next_token_out = next_tokens
                        .iter()
                        .find(|&&t| t != token_out)
                        .cloned()
                        .unwrap();

                    Box::pin(self.dfs_find_cycles(
                        next_pool,
                        token_out,
                        next_token_out,
                        visited,
                        path,
                        cycles,
                    ))
                    .await;
                }
            }
        }

        path.pop();
        if !path.is_empty() {
            visited.remove(&current_pool);
        }
    }

    pub async fn get_cycles_for_pool(
        &self,
        pool_address: Address,
    ) -> Vec<(Vec<PoolTokenPair>, Vec<PoolTokenPair>, usize)> {
        let cycles_cache = self.cycles_cache.read().await;
        let mut all_cycles = Vec::new();
        let pool_tokens = {
            let pool_map = self.pool_to_tokens.read().await;
            pool_map.get(&pool_address).cloned().unwrap_or_default()
        };
        // Get cycles for both start tokens
        for token in pool_tokens {
            if let Some(cycles) = cycles_cache.get(&(pool_address, token)) {
                all_cycles.extend(cycles.clone());
            }
        }

        all_cycles
    }

    pub async fn get_cycles_for_pool_and_token(
        &self,
        pool_address: Address,
        start_token: Address,
    ) -> Vec<(Vec<PoolTokenPair>, Vec<PoolTokenPair>, usize)> {
        let cycles_cache = self.cycles_cache.read().await;
        cycles_cache
            .get(&(pool_address, start_token))
            .cloned()
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub async fn print_all_cycles(&self) {
        let cycles_cache = self.cycles_cache.read().await;
        println!("\n===== All Detected Cycles =====");
        for ((pool, token), cycles) in cycles_cache.iter() {
            let pool_str = format!("{:?}", pool);
            let token_str = format!("{:?}", token);

            println!(
                "Pool: {}...{}, Start Token: {}...{}, Found {} cycles:",
                &pool_str[0..6],
                &pool_str[pool_str.len() - 4..],
                &token_str[0..6],
                &token_str[token_str.len() - 4..],
                cycles.len()
            );

            for (i, (rotated_cycle, _, rotation_index)) in cycles.iter().enumerate() {
                println!(
                    "Cycle #{}: {} (Rotation Index: {})",
                    i + 1,
                    format_path(rotated_cycle),
                    rotation_index
                );
            }
        }
        println!("===== End of Cycles =====\n");
    }

    #[cfg(test)]
    pub async fn print_cycles_list(&self) {
        let cycles_cache = self.cycles_cache.read().await;

        println!("\n==== Cycle List Summary ====");
        let mut cycle_count = 0;

        for ((pool, token), cycles) in cycles_cache.iter() {
            let pool_str = format!("{:?}", pool);
            let token_str = format!("{:?}", token);

            let pool_short = format!("{}..{}", &pool_str[0..6], &pool_str[pool_str.len() - 4..]);
            let token_short = format!(
                "{}..{}",
                &token_str[0..6],
                &token_str[token_str.len() - 4..]
            );

            println!(
                "Pool: {}, Start: {}, Found {} cycles:",
                pool_short,
                token_short,
                cycles.len()
            );

            for (i, (rotated_cycle, _, rotation_index)) in cycles.iter().enumerate() {
                println!(
                    "  #{}: {} (Rotation: {})",
                    i + 1,
                    format_cycle_summary(rotated_cycle),
                    rotation_index
                );
                cycle_count += 1;
            }
        }

        println!("Total cycles found: {}", cycle_count);
        println!("==== End of Cycle List ====\n");
    }

    pub async fn export_cycles_to_txt(&self, file_path: &str) -> std::io::Result<()> {
        let cycles_cache = self.cycles_cache.read().await;
        let mut file = File::create(file_path)?;

        writeln!(file, "==== Cycle List Summary ====")?;
        let mut cycle_count = 0;

        for ((pool, token), cycles) in cycles_cache.iter() {
            let pool_str = format!("{:?}", pool);
            let token_str = format!("{:?}", token);

            let pool_short = format!("{}..{}", &pool_str[0..6], &pool_str[pool_str.len() - 4..]);
            let token_short = format!(
                "{}..{}",
                &token_str[0..6],
                &token_str[token_str.len() - 4..]
            );

            writeln!(
                file,
                "Pool: {}, Start: {}, Found {} cycles:",
                pool_short,
                token_short,
                cycles.len()
            )?;

            for (i, (rotated_cycle, _, rotation_index)) in cycles.iter().enumerate() {
                writeln!(
                    file,
                    "  #{}: {} (Rotation: {})",
                    i + 1,
                    format_cycle_summary(rotated_cycle),
                    rotation_index
                )?;
                cycle_count += 1;
            }
        }

        writeln!(file, "Total cycles found: {}", cycle_count)?;
        writeln!(file, "==== End of Cycle List ====")?;

        println!("Exported cycle summary to: {}", file_path);
        Ok(())
    }

    /// Get the total number of cycles across all pools
    pub async fn get_cycle_count(&self) -> usize {
        let cycles_cache = self.cycles_cache.read().await;
        let mut total_count = 0;

        for (_, cycles) in cycles_cache.iter() {
            total_count += cycles.len();
        }

        total_count
    }

    /// Rotate a cycle to start from the first profit token
    async fn rotate_cycle_to_profit_token(
        &self,
        cycle: &[PoolTokenPair],
    ) -> Option<(Vec<PoolTokenPair>, usize)> {
        // Find the first profit token in the cycle
        for (i, pair) in cycle.iter().enumerate() {
            if self
                .profit_token_registry
                .is_profit_token(&pair.token_in)
                .await
            {
                // Rotate the cycle to start from the profit token
                let mut rotated = cycle[i..].to_vec();
                rotated.extend_from_slice(&cycle[..i]);
                return Some((rotated, i));
            }
        }
        None
    }

    pub async fn write_cycle_to_file(
        cycle: &[PoolTokenPair],
        file_path: &str,
    ) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .await?;

        let cycle_str = format_path(cycle);
        file.write_all(cycle_str.as_bytes()).await?;
        file.write_all(b"\n").await?;
        Ok(())
    }

    /// Save all data to database
    pub async fn save_to_db(&self, db: &Database) -> Result<()> {
        // Save token_to_pools
        let token_to_pools = self.token_to_pools.read().await;
        for (token, pools) in token_to_pools.iter() {
            let key = token.to_string();
            db.insert("token_to_pools", key, pools)?;
            debug!(
                "Saved token {} with {} pools to database",
                token,
                pools.len()
            );
        }

        // Save pool_to_tokens
        let pool_to_tokens = self.pool_to_tokens.read().await;
        for (pool, tokens) in pool_to_tokens.iter() {
            let key = pool.to_string();
            db.insert("pool_to_tokens", key, tokens)?;
            debug!(
                "Saved pool {} with {} tokens to database",
                pool,
                tokens.len()
            );
        }

        // Save cycles_cache
        let cycles_cache = self.cycles_cache.read().await;
        for ((pool, token), cycles) in cycles_cache.iter() {
            let key = format!("{}:{}", pool, token);
            db.insert("cycles_cache", key, cycles)?;
            debug!(
                "Saved {} cycles for pool {} and token {} to database",
                cycles.len(),
                pool,
                token
            );
        }

        // Final database snapshot to ensure everything is flushed
        db.snapshot()?;

        info!("Saved path registry data to database");
        Ok(())
    }

    /// Load data from database
    pub async fn load_from_db(&self, db: &Database) -> Result<()> {
        // Load token_to_pools
        let mut token_to_pools = self.token_to_pools.write().await;
        let iter = db.iter::<HashSet<Address>>("token_to_pools")?;
        for result in iter {
            match result {
                Ok((key, pools)) => {
                    let token = Address::from_str(&String::from_utf8(key)?)?;
                    token_to_pools.insert(token, pools);
                }
                Err(e) => log::error!("Error loading token_to_pools: {}", e),
            }
        }
        info!(
            "Loaded {} tokens from database to path registry",
            token_to_pools.len()
        );

        // Load pool_to_tokens
        let mut pool_to_tokens = self.pool_to_tokens.write().await;
        let iter = db.iter::<HashSet<Address>>("pool_to_tokens")?;
        for result in iter {
            match result {
                Ok((key, tokens)) => {
                    let pool = Address::from_str(&String::from_utf8(key)?)?;
                    pool_to_tokens.insert(pool, tokens);
                }
                Err(e) => log::error!("Error loading pool_to_tokens: {}", e),
            }
        }
        info!(
            "Loaded {} pools from database to path registry",
            pool_to_tokens.len()
        );
        // Load cycles_cache
        let mut cycles_cache = self.cycles_cache.write().await;
        let iter =
            db.iter::<Vec<(Vec<PoolTokenPair>, Vec<PoolTokenPair>, usize)>>("cycles_cache")?;
        for result in iter {
            match result {
                Ok((key, cycles)) => {
                    let key_str = String::from_utf8(key)?;
                    let parts: Vec<&str> = key_str.split(':').collect();
                    if parts.len() == 2 {
                        let pool = Address::from_str(parts[0])?;
                        let token = Address::from_str(parts[1])?;
                        cycles_cache.insert((pool, token), cycles);
                    }
                }
                Err(e) => log::error!("Error loading cycles_cache: {}", e),
            }
        }
        drop(cycles_cache);
        info!(
            "Loaded {} cycles from database",
            self.get_cycle_count().await
        );
        info!("Loaded path registry data from database");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Address, U256};

    use super::*;
    use crate::models::{
        pool::mock::MockPool,
        profit_token::{price_updater::PriceUpdater, ProfitToken},
        token::TokenRegistry,
    };

    fn create_mock_token(index: u8) -> Address {
        // Create unique addresses by using different bytes
        let mut bytes = [0u8; 20];
        bytes[0] = index;
        Address::from(bytes)
    }

    #[tokio::test]
    async fn test_path_registry() {
        // Create 10 unique mock tokens
        let tokens: Vec<Address> = (0..10).map(create_mock_token).collect();

        let token_registry = Arc::new(RwLock::new(TokenRegistry::new(1)));
        let price_updater = Arc::new(RwLock::new(
            PriceUpdater::new("ethereum".to_string(), vec![]).await,
        ));
        let profit_token_registry = Arc::new(ProfitTokenRegistry::new(
            Address::ZERO,
            token_registry.clone(),
            price_updater.clone(),
            0f64,
        ));
        profit_token_registry
            .add_token(
                tokens[1],
                ProfitToken {
                    address: tokens[1],
                    price_source: None,
                    default_price: 0.0,
                    min_profit: U256::from(0),
                    price: None,
                },
            )
            .await;
        profit_token_registry
            .add_token(
                tokens[3],
                ProfitToken {
                    address: tokens[3],
                    min_profit: U256::from(0),
                    price_source: None,
                    default_price: 0.0,
                    price: None,
                },
            )
            .await;

        profit_token_registry
            .add_token(
                tokens[4],
                ProfitToken {
                    address: tokens[4],
                    min_profit: U256::from(0),
                    price_source: None,
                    default_price: 0.0,
                    price: None,
                },
            )
            .await;
        let registry = PathRegistry::new(profit_token_registry, 4);
        println!("Starting test path registry");

        println!("Created mock tokens:");
        for (i, token) in tokens.iter().enumerate().take(10) {
            let token_str = format!("{:?}", token);
            println!(
                "  Token {}: {}...{}",
                i,
                &token_str[0..10],
                &token_str[token_str.len() - 8..]
            );
        }

        // Create mock pools with different token pairs
        let pool1 = MockPool::new_v2(
            create_mock_token(100), // Pool address
            tokens[0],
            tokens[1],
            U256::from(1000000),
            U256::from(2000000),
        );
        let pool2 = MockPool::new_v2(
            create_mock_token(101),
            tokens[1],
            tokens[2],
            U256::from(2000000),
            U256::from(3000000),
        );
        let pool3 = MockPool::new_v2(
            create_mock_token(102),
            tokens[2],
            tokens[0],
            U256::from(3000000),
            U256::from(1000000),
        );
        let pool4 = MockPool::new_v2(
            create_mock_token(103),
            tokens[0],
            tokens[1],
            U256::from(1000000),
            U256::from(2000000),
        );

        // Create some additional pools to make a more complex graph
        let pool5 = MockPool::new_v2(
            create_mock_token(104),
            tokens[3],
            tokens[2],
            U256::from(4000000),
            U256::from(1000000),
        );
        let pool6 = MockPool::new_v2(
            create_mock_token(105),
            tokens[3],
            tokens[0],
            U256::from(3000000),
            U256::from(4000000),
        );

        let pool7 = MockPool::new_v2(
            create_mock_token(106),
            tokens[4],
            tokens[5],
            U256::from(1000000),
            U256::from(2000000),
        );

        let pool8 = MockPool::new_v2(
            create_mock_token(107),
            tokens[5],
            tokens[4],
            U256::from(2000000),
            U256::from(1000000),
        );

        println!("\nCreated mock pools:");
        println!(
            "  Pool 1: Address={:?}, Tokens=[{:?}, {:?}]",
            pool1.address(),
            pool1.token0(),
            pool1.token1()
        );
        println!(
            "  Pool 2: Address={:?}, Tokens=[{:?}, {:?}]",
            pool2.address(),
            pool2.token0(),
            pool2.token1()
        );
        println!(
            "  Pool 3: Address={:?}, Tokens=[{:?}, {:?}]",
            pool3.address(),
            pool3.token0(),
            pool3.token1()
        );
        println!(
            "  Pool 4: Address={:?}, Tokens=[{:?}, {:?}]",
            pool4.address(),
            pool4.token0(),
            pool4.token1()
        );
        println!(
            "  Pool 5: Address={:?}, Tokens=[{:?}, {:?}]",
            pool5.address(),
            pool5.token0(),
            pool5.token1()
        );
        println!(
            "  Pool 6: Address={:?}, Tokens=[{:?}, {:?}]",
            pool6.address(),
            pool6.token0(),
            pool6.token1()
        );
        println!(
            "  Pool 7: Address={:?}, Tokens=[{:?}, {:?}]",
            pool7.address(),
            pool7.token0(),
            pool7.token1()
        );
        println!(
            "  Pool 8: Address={:?}, Tokens=[{:?}, {:?}]",
            pool8.address(),
            pool8.token0(),
            pool8.token1()
        );

        // Add pools to registry
        println!("\nAdding pools to registry...");
        registry.add_pool(&pool1).await;
        registry.add_pool(&pool2).await;
        registry.add_pool(&pool3).await;
        registry.add_pool(&pool4).await;
        registry.add_pool(&pool5).await;
        registry.add_pool(&pool6).await;
        registry.add_pool(&pool7).await;
        registry.add_pool(&pool8).await;

        // Find cycles
        println!("\nFinding cycles...");
        registry.find_all_cycles().await;

        // Print all cycles with our nice formatter
        println!("\nPrinting detailed cycles...");
        registry.print_all_cycles().await;

        // Print concise cycle list
        println!("\nPrinting concise cycle list...");
        registry.print_cycles_list().await;

        // Export cycles to txt file
        println!("\nExporting cycles to text file...");
        match registry
            .export_cycles_to_txt("./target/cycles_summary.txt")
            .await
        {
            Ok(_) => println!("Successfully exported cycles to text file"),
            Err(e) => println!("Error exporting cycles to text file: {}", e),
        }

        // Get cycles for pool and verify
        let cycles = registry
            .get_cycles_for_pool_and_token(pool1.address(), pool1.token0())
            .await;
        println!("\nVerifying cycles for pool1, token0:");
        if cycles.is_empty() {
            println!("  No cycles found!");
        } else {
            for (i, (rotated_cycle, original_cycle, rotation_index)) in cycles.iter().enumerate() {
                println!(
                    "  Cycle #{}: Rotated: {} | Original: {} | Rotation Index: {}",
                    i + 1,
                    format_cycle_summary(rotated_cycle),
                    format_cycle_summary(original_cycle),
                    rotation_index
                );
            }
        }

        // Count the number of cycles for verification
        let count = cycles.len();
        assert_eq!(count, 5);
    }
}
