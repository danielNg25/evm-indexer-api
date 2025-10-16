use crate::models::pool::base::Topic;
use crate::models::pool::PoolRegistry;
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::Address;
use alloy::providers::Provider;
use anyhow::Result;
use log::{debug, error, info};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use super::fetch_events;

pub struct PoolUpdaterLatestBlock<P: Provider + Send + Sync + 'static> {
    network_id: u64,
    provider: Arc<P>,
    pool_registry: Arc<PoolRegistry>,
    // metrics: Arc<RwLock<Metrics>>,
    max_blocks_per_batch: u64,
    // swap_event_tx: mpsc::Sender<PendingEvent>,
    topics: Arc<Vec<Topic>>,
    profitable_topics: Arc<HashSet<Topic>>,
}

impl<P: Provider + Send + Sync + 'static> PoolUpdaterLatestBlock<P> {
    pub async fn new(
        provider: Arc<P>,
        pool_registry: Arc<PoolRegistry>,
        // metrics: Arc<RwLock<Metrics>>,
        //swap_event_tx: mpsc::Sender<PendingEvent>,
        start_block: u64,
        max_blocks_per_batch: u64,
    ) -> Self {
        let network_id = pool_registry.get_network_id();
        // Initialize the last_processed_block in the registry if it's currently 0
        tokio::spawn({
            let pool_registry = Arc::clone(&pool_registry);
            async move {
                let current_block = pool_registry.get_last_processed_block().await;
                if current_block == 0 {
                    pool_registry.set_last_processed_block(start_block).await;
                    info!(
                        "CHAIN ID: {} Initialized last processed block to {}",
                        network_id, start_block
                    );
                } else if start_block > 0 && start_block > current_block {
                    // Override existing block if a higher start_block is provided
                    pool_registry.set_last_processed_block(start_block).await;
                    info!(
                        "CHAIN ID: {} Updated last processed block from {} to {}",
                        network_id, current_block, start_block
                    );
                }
            }
        });
        Self {
            network_id,
            provider,
            pool_registry: pool_registry.clone(),
            //metrics: metrics.clone(),
            max_blocks_per_batch,
            //swap_event_tx,
            topics: Arc::new(pool_registry.get_topics().await.clone()),
            profitable_topics: Arc::new(pool_registry.get_profitable_topics().await.clone()),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        loop {
            // Get latest block number with retry logic
            let mut backoff = Duration::from_millis(50);
            let max_backoff = Duration::from_millis(500);
            let latest_block = loop {
                match self.provider.get_block_number().await {
                    Ok(block) => {
                        break block;
                    }
                    Err(e) => {
                        error!(
                            "CHAIN ID: {} Error fetching block number, retrying in {}s: {}",
                            self.network_id,
                            backoff.as_secs(),
                            e
                        );
                        tokio::time::sleep(backoff).await;
                        backoff = std::cmp::min(backoff * 2, max_backoff);
                    }
                }
            };

            // Get the last processed block from registry
            let last_processed_block = self.pool_registry.get_last_processed_block().await;

            // Process blocks in batches up to the latest confirmed block
            let mut current_block = last_processed_block + 1;
            if current_block > latest_block {
                debug!(
                    "Waiting for new blocks. Current: {}, Latest: {}",
                    current_block, latest_block
                );
                tokio::time::sleep(Duration::from_millis(50)).await;
                continue;
            }

            while current_block <= latest_block {
                let batch_end =
                    std::cmp::min(current_block + self.max_blocks_per_batch - 1, latest_block);

                // Process pools for confirmed blocks
                match proccess_pools(
                    self.network_id,
                    &self.provider,
                    &self.pool_registry,
                    // &self.metrics,
                    //&self.swap_event_tx,
                    BlockNumberOrTag::Number(current_block),
                    BlockNumberOrTag::Number(batch_end),
                    batch_end == latest_block,
                    self.topics.clone(),
                    self.profitable_topics.clone(),
                )
                .await
                {
                    Ok(_) => {
                        // Update last processed block in registry
                        self.pool_registry.set_last_processed_block(batch_end).await;
                        info!(
                            "CHAIN ID: {} Successfully processed blocks: {} - {}",
                            self.network_id, current_block, batch_end
                        );
                    }
                    Err(e) => {
                        error!(
                            "CHAIN ID: {} Error processing blocks {} - {}: {}",
                            self.network_id, current_block, batch_end, e
                        );
                        // Don't update last_processed_block on error
                        break;
                    }
                }

                current_block = batch_end + 1;
            }

            // Add a small delay between iterations to prevent tight loops
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

async fn proccess_pools<P: Provider + Send + Sync + 'static>(
    network_id: u64,
    provider: &Arc<P>,
    pool_registry: &Arc<PoolRegistry>,
    //metrics: &Arc<RwLock<Metrics>>,
    //swap_event_tx: &mpsc::Sender<PendingEvent>,
    from_block: BlockNumberOrTag,
    to_block: BlockNumberOrTag,
    _is_latest_block: bool,
    topics: Arc<Vec<Topic>>,
    _profitable_topics: Arc<HashSet<Topic>>,
) -> Result<()> {
    let addresses: Vec<Address> = pool_registry.get_all_addresses().await;
    let addresses_len = addresses.len();
    if addresses_len == 0 {
        return Ok(());
    }

    let mut backoff = Duration::from_millis(50);
    let max_backoff = Duration::from_millis(500);

    let topics = topics.clone().to_vec();
    loop {
        match fetch_events(
            provider,
            addresses.clone(),
            topics.clone(),
            from_block,
            to_block,
        )
        .await
        {
            Ok(events) => {
                // let mut swap_events = Vec::new();
                debug!(
                    "CHAIN ID: {} Processing {} events from {} to {}",
                    network_id,
                    events.len(),
                    from_block.as_number().unwrap(),
                    to_block.as_number().unwrap()
                );
                for event in events {
                    if let Some(pool) = pool_registry.get_pool(&event.address()).await {
                        if let Err(e) = pool.write().await.apply_log(&event) {
                            error!(
                                "CHAIN ID: {} Error applying event {} for pool {}, event {}",
                                network_id,
                                e,
                                event.address(),
                                event.transaction_hash.unwrap()
                            );
                        }

                        // SKIP FOR NOW
                        // if is_latest_block && profitable_topics.contains(event.topic0().unwrap()) {
                        //     swap_events.push(event);
                        // }
                        //     swap_events.push(event);
                        // }
                    }
                }
                // SKIP FOR NOW
                // if is_latest_block && from_block == to_block {
                //     for event in swap_events {
                //         let tx_hash = event.transaction_hash.unwrap();
                //         let log_index = event.log_index.unwrap();
                //         let mut guard = metrics.write().await;
                //         guard.add_opportunity(tx_hash, log_index, received_at);
                //         guard.set_proccessed_at(
                //             tx_hash,
                //             log_index,
                //             Utc::now().timestamp_millis() as u64,
                //         );
                //         drop(guard);
                //         if let Err(e) = swap_event_tx
                //             .send(PendingEvent {
                //                 event,
                //                 modified_pools: Arc::new(RwLock::new(HashMap::new())),
                //             })
                //             .await
                //         {
                //             error!("Error sending swap event to simulator: {}", e);
                //         }
                //     }
                // }
                break;
            }
            Err(e) => {
                error!(
                    "CHAIN ID: {} Error fetching events, retrying in {}s: {}",
                    network_id,
                    backoff.as_secs(),
                    e
                );
                tokio::time::sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        }
    }

    Ok(())
}
