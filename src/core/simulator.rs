use crate::blockchain::{IPancakeV3Pool, IUniswapV2Pair, IUniswapV3Pool, PendingEvent};
use crate::models::path::{PathRegistry, PoolTokenPair};
use crate::models::pool::erc4626::ERC4626Pool;
use crate::models::pool::{PoolRegistry, PoolType};
use crate::models::profit_token::ProfitTokenRegistry;
use crate::utils::metrics::Metrics;
use crate::utils::utils::OpportunityStatus;
use crate::PoolInterface;
use alloy::primitives::{Address, TxHash, U256};
use alloy::sol_types::SolEvent;
use anyhow::Result;
use chrono::Utc;
use dashmap::DashMap;
use futures::future::join_all;
use log::{debug, error, info};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock; // For parallel iteration

pub struct Opportunity {
    pub status: OpportunityStatus,
    pub cycle: Vec<PoolTokenPair>,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub profit_token: Address,
    pub original_token_amount: U256,
    pub original_token_profit: U256,
    pub profit_token_amount: U256,
    pub profit: U256,
    pub transaction_hash: TxHash,
    pub log_index: u64,
    pub source_pool: Address,
    pub strategy_name: Option<String>,
}

/// Search strategy for finding optimal amounts
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SearchStrategy {
    /// Fast search with fewer samples and narrower search regions
    /// Prioritizes speed over finding the exact optimal amount
    Fast,

    /// Balanced approach with moderate samples and search regions
    /// Good balance between speed and accuracy
    Normal,

    /// Thorough search with more samples and wider search regions
    /// Prioritizes finding the exact optimal amount over speed
    Slow,
}

impl Default for SearchStrategy {
    fn default() -> Self {
        SearchStrategy::Normal
    }
}

/// Configuration for the simulator
#[derive(Clone)]
pub struct SimulatorConfig {
    /// Maximum number of iterations for binary search
    pub max_iterations: u32,
    /// Search strategy for finding optimal amounts
    pub search_strategy: SearchStrategy,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            search_strategy: SearchStrategy::Normal,
        }
    }
}

// This type definition and struct have been removed as they're no longer needed
// The functionality is now handled by:
// 1. token_max_outputs (FxHashMap) for early cut-off
// 2. memo_cache for caching calculations

pub struct ArbitrageSimulator {
    pool_registry: Arc<PoolRegistry>,
    path_registry: Arc<PathRegistry>,
    profit_token_registry: Arc<ProfitTokenRegistry>,
    config: SimulatorConfig, // Added config field
    opportunity_tx: Sender<Opportunity>,
    metrics: Arc<RwLock<Metrics>>,
    strategy_name: String, // Added strategy name
}

impl ArbitrageSimulator {
    pub fn new(
        pool_registry: Arc<PoolRegistry>,
        path_registry: Arc<PathRegistry>,
        profit_token_registry: Arc<ProfitTokenRegistry>,
        config: SimulatorConfig, // Added config parameter
        opportunity_tx: Sender<Opportunity>,
        metrics: Arc<RwLock<Metrics>>,
        strategy_name: String, // Added strategy name parameter
    ) -> Self {
        Self {
            pool_registry,
            path_registry,
            profit_token_registry,
            config, // Store config
            opportunity_tx,
            metrics,
            strategy_name,
        }
    }

    /// Handle a Swap event from any pool
    pub async fn handle_swap_event(&self, event: PendingEvent) -> Result<()> {
        let start_token: Address;
        let max_amount: U256;
        let log = &event.event; // Adjust based on actual PendingEvent struct

        let block_number = log.block_number.unwrap_or(0);
        let block_timestamp = log
            .block_timestamp
            .unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
        let pool_address = log.address();
        let transaction_hash = log.transaction_hash.unwrap_or(TxHash::ZERO);
        let log_index = log.log_index.unwrap_or(0);

        // Create a global memoization cache for this event
        // This caches output amounts across all cycles for the same (pool, tokenIn, amountIn)
        // Using DashMap for lock-free concurrent access
        let memo_cache: Arc<DashMap<(Address, Address, U256), U256>> = Arc::new(DashMap::new());

        match log.topic0() {
            Some(&IUniswapV2Pair::Swap::SIGNATURE_HASH) => {
                let swap_data: IUniswapV2Pair::Swap = log.log_decode()?.inner.data;
                debug!(
                    "Processing V2 swap event: in0={:?}, in1={:?}, out0={:?}, out1={:?}",
                    swap_data.amount0In,
                    swap_data.amount1In,
                    swap_data.amount0Out,
                    swap_data.amount1Out
                );
                // Get pool and determine swap direction
                (start_token, max_amount) = {
                    let pool = self.pool_registry.get_pool(&pool_address).await.unwrap();
                    let pool_guard = pool.read().await;
                    if swap_data.amount0In > U256::ZERO && swap_data.amount1Out > U256::ZERO {
                        (pool_guard.token1(), swap_data.amount1Out)
                    } else {
                        (pool_guard.token0(), swap_data.amount0Out)
                    }
                };
            }
            Some(&IUniswapV3Pool::Swap::SIGNATURE_HASH) => {
                let swap_data: IUniswapV3Pool::Swap = log.log_decode()?.inner.data;
                debug!(
                    "Processing V3 swap event: amount0={:?}, amount1={:?}",
                    swap_data.amount0, swap_data.amount1
                );
                let zero_for_one = swap_data.amount0.is_positive();

                // Get pool and determine swap direction
                (start_token, max_amount) = {
                    let pool = self.pool_registry.get_pool(&pool_address).await.unwrap();
                    let pool_guard: tokio::sync::RwLockReadGuard<
                        '_,
                        Box<dyn PoolInterface + Send + Sync>,
                    > = pool.read().await;
                    if zero_for_one {
                        (pool_guard.token1(), swap_data.amount1.unsigned_abs())
                    } else {
                        (pool_guard.token0(), swap_data.amount0.unsigned_abs())
                    }
                };
            }
            Some(&IPancakeV3Pool::Swap::SIGNATURE_HASH) => {
                let swap_data: IPancakeV3Pool::Swap = log.log_decode()?.inner.data;
                debug!(
                    "Processing V3 swap event: amount0={:?}, amount1={:?}",
                    swap_data.amount0, swap_data.amount1
                );
                let zero_for_one = swap_data.amount0.is_positive();

                // Get pool and determine swap direction
                (start_token, max_amount) = {
                    let pool = self.pool_registry.get_pool(&pool_address).await.unwrap();
                    let pool_guard: tokio::sync::RwLockReadGuard<
                        '_,
                        Box<dyn PoolInterface + Send + Sync>,
                    > = pool.read().await;
                    if zero_for_one {
                        (pool_guard.token1(), swap_data.amount1.unsigned_abs())
                    } else {
                        (pool_guard.token0(), swap_data.amount0.unsigned_abs())
                    }
                };
            }
            _ => {
                debug!("Ignoring non-swap event: {:?}", event.event);
                return Ok(());
            }
        }
        let start = Instant::now();

        // Get all cycles starting with this token
        let cycle_pairs = {
            self.path_registry
                .get_cycles_for_pool_and_token(pool_address, start_token)
                .await
        };

        // Build a pool snapshot for all pools referenced in these cycles
        // This reduces lock contention by creating a local copy of only the needed pools
        let mut needed_pools = std::collections::HashSet::new();
        for (_, original_cycle, _) in &cycle_pairs {
            for pair in original_cycle {
                needed_pools.insert(pair.pool);
            }
        }

        // Create a local pool snapshot
        let mut pool_snapshot: HashMap<Address, Box<dyn PoolInterface + Send + Sync>> =
            HashMap::new();

        // First check modified_pools (from the event)
        {
            let modified_guard = event.modified_pools.read().await;
            for &addr in &needed_pools {
                if let Some(pool) = modified_guard.get(&addr) {
                    // Clone the pool to our local snapshot
                    pool_snapshot.insert(addr, pool.clone_box());
                }
            }
        }

        // Then get any remaining pools from the registry
        for &addr in &needed_pools {
            if !pool_snapshot.contains_key(&addr) {
                if let Some(pool_arc) = self.pool_registry.get_pool(&addr).await {
                    let pool_guard = pool_arc.read().await;
                    pool_snapshot.insert(addr, pool_guard.clone_box());
                }
            }
        }

        // Convert to Arc for sharing
        let pool_snapshot = Arc::new(pool_snapshot);

        // We no longer need cut_off_data as we're using:
        // 1. Local token_max_outputs for early cut-off
        // 2. Global memo_cache for caching calculations

        // Clone self's fields for tasks
        // Spawn tasks for each cycle simulation
        let tasks: Vec<_> = cycle_pairs
            .into_iter()
            .map(|(rotated_cycle, original_cycle, rotation_index)| {
                let pool_snapshot = Arc::clone(&pool_snapshot);
                let memo_cache = Arc::clone(&memo_cache);
                let original_token = original_cycle[0].token_in;
                let profit_token = rotated_cycle[0].token_in;
                let cycle_clone = original_cycle.clone(); // Clone for task ownership
                let rotated_cycle_clone = rotated_cycle.clone(); // Clone for result
                let strategy_name = self.strategy_name.clone(); // Clone the strategy name

                let config = self.config.clone();

                tokio::spawn(async move {
                    let result = simulate_cycle_with_snapshot(
                        &cycle_clone,
                        max_amount,
                        rotation_index,
                        &pool_snapshot,
                        &config,
                        &memo_cache,
                    )
                    .await;

                    match result {
                        Ok(Some((
                            original_token_amount,
                            original_token_profit,
                            profit_token_amount,
                        ))) => {
                            debug!(
                                "Found potential opportunity with {}: profit {} amount {}",
                                original_token, original_token_profit, original_token_amount
                            );
                            Some((
                                original_token_profit,
                                Opportunity {
                                    status: OpportunityStatus::None,
                                    cycle: rotated_cycle_clone,
                                    block_number,
                                    block_timestamp,
                                    profit_token,
                                    original_token_amount,
                                    original_token_profit,
                                    profit_token_amount,
                                    profit: U256::ZERO,
                                    transaction_hash,
                                    log_index,
                                    source_pool: pool_address,
                                    strategy_name: Some(strategy_name),
                                },
                            ))
                        }
                        Ok(None) => {
                            debug!(
                                "No profitable opportunity found for cycle: {:?}",
                                cycle_clone
                            );
                            None
                        }
                        Err(e) => {
                            error!("Error simulating cycle: {}", e);
                            None
                        }
                    }
                })
            })
            .collect();

        // Await all tasks and collect results
        let results: Vec<_> = join_all(tasks)
            .await
            .into_iter()
            .filter_map(|result| result.ok().flatten())
            .collect();

        // Find best opportunity
        let best_opportunity = results
            .into_iter()
            .max_by_key(|(profit, _)| *profit)
            .map(|(_, opp)| opp);
        let duration = start.elapsed();
        info!(
            "Time taken: {:?} for tx hash {}, memo cache size: {}",
            duration,
            transaction_hash,
            memo_cache.len()
        );

        // Send only the best opportunity if found
        if let Some(mut opportunity) = best_opportunity {
            // Directly use the synchronous function for final verification
            let (profit, _, steps) = simulate_amount_sync(
                &opportunity.cycle,
                opportunity.profit_token_amount,
                0,
                pool_snapshot.as_ref(),
                Some(&memo_cache),
            )?;

            if let Some(min_profit) = self
                .profit_token_registry
                .get_min_profit(&opportunity.profit_token)
                .await
            {
                if profit > min_profit {
                    opportunity.profit = profit;
                    info!(
                        "Found profitable opportunity with {}: profit {} amount {}",
                        opportunity.profit_token,
                        opportunity.profit,
                        opportunity.profit_token_amount,
                    );
                    self.metrics.write().await.set_steps(
                        opportunity.transaction_hash,
                        opportunity.log_index,
                        steps,
                    );
                } else {
                    debug!(
                            "Skipping opportunity not satisfying min profit with {}: profit {} amount {}",
                            opportunity.profit_token, profit, opportunity.profit_token_amount
                        );
                    return Ok(());
                }
            } else {
                debug!("No min profit found for {}", opportunity.profit_token);
                return Ok(());
            }

            self.metrics.write().await.set_simulated_at(
                opportunity.transaction_hash,
                opportunity.log_index,
                Utc::now().timestamp_millis() as u64,
            );

            if let Err(e) = self.opportunity_tx.send(opportunity).await {
                error!("Failed to send opportunity: {}", e);
            }
        }
        Ok(())
    }

    // These methods have been removed as they're no longer needed
    // The code now directly uses simulate_cycle_with_snapshot and simulate_amount_sync
}

/// Simulate a cycle to find maximum profit using a pre-built pool snapshot
#[inline]
async fn simulate_cycle_with_snapshot(
    cycle: &[PoolTokenPair],
    max_amount: U256,
    rotation_index: usize,
    pool_snapshot: &Arc<HashMap<Address, Box<dyn PoolInterface + Send + Sync>>>,
    config: &SimulatorConfig,
    memo_cache: &Arc<DashMap<(Address, Address, U256), U256>>,
) -> Result<Option<(U256, U256, U256)>> {
    // If max_amount is too small, return early
    if max_amount <= U256::ONE {
        return Ok(None);
    }

    // We no longer need a local cache since we're using the global memo cache

    // Track best profit
    let mut best_profit = U256::ZERO;
    let mut best_amount = U256::ZERO;
    let mut best_profit_token_amount = U256::ZERO;

    // Track search state
    let mut iterations = 0;

    // Configure search parameters based on strategy
    let (sample_capacity, min_samples) = match config.search_strategy {
        SearchStrategy::Fast => {
            // Fast: Fewer samples
            (6, 3)
        }
        SearchStrategy::Normal => {
            // Normal: Moderate samples
            (10, 5)
        }
        SearchStrategy::Slow => {
            // Slow: More samples
            (15, 8)
        }
    };

    // Generate better distributed sample points that work well for token amounts (where 1 = 10^18)
    let mut sample_points = Vec::with_capacity(sample_capacity);

    // Always include these key points
    sample_points.push(U256::ONE); // Smallest possible amount
    sample_points.push(max_amount); // Maximum amount

    // Add the midpoint (this is where naive binary search would start)
    sample_points.push(max_amount / U256::from(2));

    // For normal and slow strategies, add quartiles
    if config.search_strategy != SearchStrategy::Fast {
        sample_points.push(max_amount / U256::from(4));
        sample_points.push(max_amount * U256::from(3) / U256::from(4));
    }

    // For slow strategy, add even more division points
    if config.search_strategy == SearchStrategy::Slow {
        sample_points.push(max_amount / U256::from(8));
        sample_points.push(max_amount * U256::from(3) / U256::from(8));
        sample_points.push(max_amount * U256::from(5) / U256::from(8));
        sample_points.push(max_amount * U256::from(7) / U256::from(8));

        // Add even finer divisions for very thorough search
        sample_points.push(max_amount / U256::from(16));
        sample_points.push(max_amount * U256::from(15) / U256::from(16));
    }

    // Add logarithmic points between 1 and max_amount/16 to catch small optimal amounts
    // These are especially important for tokens with high value
    if max_amount > U256::from(16) {
        let log_max = max_amount / U256::from(16); // Only use logarithmic sampling for the lower 1/16 of the range
        let mut point = U256::from(10); // Start at 10 instead of 1

        while point < log_max {
            sample_points.push(point);
            // Use a larger multiplier for bigger jumps
            point = point.saturating_mul(U256::from(100));
        }
    }

    // Sort and deduplicate sample points
    sample_points.sort();
    sample_points.dedup();

    // Ensure we have at least min_samples points
    if sample_points.len() < min_samples && max_amount > U256::from(100) {
        // Calculate how many additional points we need
        let additional_needed = min_samples - sample_points.len();
        if additional_needed > 0 {
            // Create evenly spaced points across the range
            let step = max_amount / U256::from(additional_needed + 1);
            for i in 1..=additional_needed {
                let new_point = step * U256::from(i);
                if new_point > U256::ONE && new_point < max_amount {
                    sample_points.push(new_point);
                }
            }
            // Sort again after adding new points
            sample_points.sort();
            sample_points.dedup();
        }
    }

    // Evaluate all sample points
    let mut sample_results = Vec::with_capacity(sample_points.len());
    for &amount in &sample_points {
        iterations += 1;
        if iterations > config.max_iterations {
            break;
        }

        // Run simulation
        let (profit, profit_token_amount, _) = simulate_amount_sync(
            cycle,
            amount,
            rotation_index,
            pool_snapshot.as_ref(),
            Some(memo_cache),
        )?;

        // Track result
        sample_results.push((amount, profit, profit_token_amount));

        // Update best seen
        if profit > best_profit {
            best_profit = profit;
            best_amount = amount;
            best_profit_token_amount = profit_token_amount;
        }
    }

    // If we found no profit in the samples, return early
    if best_profit == U256::ZERO {
        return Ok(None);
    }

    // Find the most promising region based on sample results
    // Sort by profit to find the best point
    sample_results.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by profit (descending)

    // Second phase: Focused binary search around the most promising point
    if iterations < config.max_iterations && sample_results.len() > 0 {
        // Get the best point from sampling
        let (best_sample, _, _) = sample_results[0];

        // Define search bounds around this point with radius based on strategy
        let search_radius_factor = U256::from(2); // 50% radius

        // Use wider radius for more thorough search
        let search_radius = if best_sample > U256::from(4) {
            best_sample.saturating_mul(search_radius_factor)
                / U256::from(search_radius_factor + U256::ONE)
        } else {
            U256::ONE
        };

        let low = if best_sample > search_radius {
            best_sample - search_radius
        } else {
            U256::ONE
        };
        let high = std::cmp::min(best_sample + search_radius, max_amount);

        // Only continue if we have a meaningful range to search
        if high > low.saturating_add(U256::from(10)) {
            let mut search_low = low;
            let mut search_high = high;

            // Traditional binary/ternary search in this focused region
            while search_low <= search_high && iterations < config.max_iterations {
                iterations += 1;

                // Avoid underflow when low == high
                if search_low == search_high {
                    let (profit, profit_token_amount, _) = simulate_amount_sync(
                        cycle,
                        search_low,
                        rotation_index,
                        pool_snapshot.as_ref(),
                        Some(memo_cache),
                    )?;

                    if profit > best_profit {
                        best_profit = profit;
                        best_amount = search_low;
                        best_profit_token_amount = profit_token_amount;
                    }
                    break;
                }

                // Test points based on strategy
                let (mid1, mid2) = match config.search_strategy {
                    SearchStrategy::Fast => {
                        // Fast: Just test the midpoint (binary search)
                        let mid = search_low + (search_high - search_low) / U256::from(2);
                        (mid, mid)
                    }
                    _ => {
                        // Normal/Slow: Test two points (ternary search)
                        let mid1 = search_low + (search_high - search_low) / U256::from(3);
                        let mid2 =
                            search_low + (search_high - search_low) * U256::from(2) / U256::from(3);
                        (mid1, mid2)
                    }
                };

                // Get or create cache entry for mid1
                // Evaluate mid1
                let (profit1, profit_token_amount1, _) = simulate_amount_sync(
                    cycle,
                    mid1,
                    rotation_index,
                    pool_snapshot.as_ref(),
                    Some(memo_cache),
                )?;

                // Update best if applicable
                if profit1 > best_profit {
                    best_profit = profit1;
                    best_amount = mid1;
                    best_profit_token_amount = profit_token_amount1;
                }

                // For Fast strategy, we're doing binary search
                if config.search_strategy == SearchStrategy::Fast {
                    // Binary search decision
                    // For binary search, we only need to check which direction to go
                    let mid_plus_one = mid1 + U256::ONE;
                    if mid_plus_one <= search_high {
                        // Check if profit is increasing or decreasing after mid
                        let (profit_plus, profit_token_plus, _) = simulate_amount_sync(
                            cycle,
                            mid_plus_one,
                            rotation_index,
                            pool_snapshot.as_ref(),
                            Some(memo_cache),
                        )?;

                        if profit_plus > profit1 {
                            // Profit increasing, search right half
                            search_low = mid1 + U256::ONE;
                        } else {
                            // Profit decreasing, search left half
                            search_high = mid1;
                        }

                        // Update best if applicable
                        if profit_plus > best_profit {
                            best_profit = profit_plus;
                            best_amount = mid_plus_one;
                            best_profit_token_amount = profit_token_plus;
                        }
                    } else {
                        // Can't check beyond mid, just stop
                        break;
                    }
                } else {
                    // Normal/Slow: Evaluate mid2 for ternary search
                    let (profit2, profit_token_amount2, _) = simulate_amount_sync(
                        cycle,
                        mid2,
                        rotation_index,
                        pool_snapshot.as_ref(),
                        Some(memo_cache),
                    )?;

                    // Update best if applicable
                    if profit2 > best_profit {
                        best_profit = profit2;
                        best_amount = mid2;
                        best_profit_token_amount = profit_token_amount2;
                    }

                    // Ternary search decision
                    if profit1 >= profit2 {
                        // Peak is likely at mid1 or left of mid1
                        search_high = mid2 - U256::ONE;
                    } else {
                        // Peak is likely right of mid1
                        search_low = mid1 + U256::ONE;
                    }
                }
            }
        }
    }

    // We no longer need to update a shared cache since we're using the global memo cache

    if best_profit > U256::ZERO {
        return Ok(Some((best_amount, best_profit, best_profit_token_amount)));
    }

    Ok(None)
}

/// Simulate a specific amount through the cycle - synchronous version with no awaits
/// This is the hot path that needs to be as efficient as possible
#[inline]
fn simulate_amount_sync(
    cycle: &[PoolTokenPair],
    amount_in: U256,
    rotation_index: usize,
    pool_snapshot: &HashMap<Address, Box<dyn PoolInterface + Send + Sync>>,
    memo_cache: Option<&Arc<DashMap<(Address, Address, U256), U256>>>,
) -> Result<(U256, U256, Vec<U256>)> {
    let mut current_amount = amount_in;
    let mut current_token: Address = cycle[0].token_in;
    let mut profit_token_amount: U256 = U256::ZERO;

    // Pre-allocate steps vector with capacity to avoid reallocations
    let mut steps: Vec<U256> = Vec::with_capacity(cycle.len() + 1);
    steps.push(current_amount);

    // Create a local token max tracker for this simulation path
    // This avoids HashMap lookups in the cut-off check hot path
    let expected_tokens = std::cmp::min(cycle.len(), 8); // Most cycles involve <8 unique tokens
    let mut token_max_outputs: FxHashMap<Address, U256> =
        FxHashMap::with_capacity_and_hasher(expected_tokens, Default::default());

    // Track if we've seen any token more than once in the path
    // If a token only appears once, we don't need cut-off checks for it
    let mut token_frequencies: FxHashMap<Address, u8> =
        FxHashMap::with_capacity_and_hasher(expected_tokens, Default::default());
    for pair in cycle {
        *token_frequencies.entry(pair.token_out).or_insert(0) += 1;
    }

    for (i, pair) in cycle.iter().enumerate() {
        if i == rotation_index {
            profit_token_amount = current_amount;
        }

        // We now use only the memo cache, no more cut_off_data
        let output_amount = if let Some(cache) = memo_cache {
            // Create a key for the memo cache: (pool_address, token_in, amount_in)
            let key = (pair.pool, current_token, current_amount);

            // DashMap provides lock-free access - much simpler and more efficient!
            if let Some(cached_value) = cache.get(&key) {
                // Cache hit!
                *cached_value.value()
            } else {
                // Cache miss - calculate and store
                // Use the pool from our snapshot - no async needed
                let pool = match pool_snapshot.get(&pair.pool) {
                    Some(pool) => pool,
                    None => {
                        // If pool not found in snapshot, return zero profit
                        return Ok((U256::ZERO, U256::ZERO, steps));
                    }
                };

                // Calculate output directly from the pool in our snapshot - no async
                let calculated_amount = match pool.pool_type() {
                    PoolType::UniswapV2 => {
                        match pool.calculate_output(&current_token, current_amount) {
                            Ok(amount) => amount,
                            Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                        }
                    }
                    PoolType::UniswapV3 => {
                        match pool.calculate_output(&current_token, current_amount) {
                            Ok(amount) => amount,
                            Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                        }
                    }
                    PoolType::ERC4626(pool_type) => match pool_type {
                        ERC4626Pool::VerioIP => {
                            match pool.calculate_output(&current_token, current_amount) {
                                Ok(amount) => amount,
                                Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                            }
                        }
                    },
                };

                // Store in memo cache for future use - no locking needed!
                cache.insert(key, calculated_amount);
                calculated_amount
            }
        } else {
            // No memo cache available, calculate directly
            let pool = match pool_snapshot.get(&pair.pool) {
                Some(pool) => pool,
                None => {
                    // If pool not found in snapshot, return zero profit
                    return Ok((U256::ZERO, U256::ZERO, steps));
                }
            };

            // Calculate output directly from the pool in our snapshot - no async
            match pool.pool_type() {
                PoolType::UniswapV2 => {
                    match pool.calculate_output(&current_token, current_amount) {
                        Ok(amount) => amount,
                        Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                    }
                }
                PoolType::UniswapV3 => {
                    match pool.calculate_output(&current_token, current_amount) {
                        Ok(amount) => amount,
                        Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                    }
                }
                PoolType::ERC4626(pool_type) => match pool_type {
                    ERC4626Pool::VerioIP => {
                        match pool.calculate_output(&current_token, current_amount) {
                            Ok(amount) => amount,
                            Err(_e) => return Ok((U256::ZERO, U256::ZERO, steps)),
                        }
                    }
                },
            }
        };

        // No need for cut_off_data anymore

        // Check if this token appears multiple times in the path
        if token_frequencies.get(&pair.token_out).copied().unwrap_or(0) > 1 {
            // Enhanced early cut-off for tokens appearing multiple times
            // Check against our local max tracker first (faster than HashMap lookup)
            if let Some(&max_seen) = token_max_outputs.get(&pair.token_out) {
                if output_amount < max_seen {
                    // We've found a worse path to this token - cut off early
                    return Ok((U256::ZERO, U256::ZERO, steps));
                }
            }

            // Update local max tracker
            token_max_outputs.insert(pair.token_out, output_amount);
        }

        current_amount = output_amount;
        current_token = pair.token_out;
        steps.push(current_amount);
    }

    // Calculate profit
    if current_amount > amount_in {
        Ok((current_amount - amount_in, profit_token_amount, steps))
    } else {
        Ok((U256::ZERO, U256::ZERO, steps))
    }
}
