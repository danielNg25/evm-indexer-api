use crate::blockchain::IUniswapV3Pool;
use crate::models::pool::base::PoolInterface;
use crate::models::pool::erc4626::erc4626_standard::fetch_erc4626_pool;
use crate::models::pool::v2::fetch_v2_pool;
use crate::models::pool::v3::fetch_v3_pool;
use crate::models::pool::PoolRegistry;
use crate::models::pool::PoolType;
use crate::models::token::TokenRegistry;
use alloy::eips::{BlockId, BlockNumberOrTag};
use alloy::primitives::Address;
use alloy::providers::Provider;
use anyhow::Result;
use log::info;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Identifies the type o
pub async fn identify_pool_type<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_address: Address,
) -> Result<PoolType> {
    // Try to read the fee() function which exists in V3 but not V2

    let pair_instance = IUniswapV3Pool::new(pool_address, &provider);
    let fee_call = pair_instance.liquidity().into_transaction_request();

    match provider.call(fee_call).await {
        Ok(_) => Ok(PoolType::UniswapV3),
        Err(_) => Ok(PoolType::UniswapV2),
    }
}

/// Main function to fetch pool data
pub async fn fetch_pool<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_address: Address,
    block_number: BlockId,
    pool_type: PoolType,
    token_registry: &Arc<RwLock<TokenRegistry>>,
    multicall_address: Address,
) -> Result<Box<dyn PoolInterface>> {
    match pool_type {
        PoolType::UniswapV2 => {
            let pool = fetch_v2_pool(
                provider,
                pool_address,
                block_number,
                token_registry,
                multicall_address,
            )
            .await?;
            Ok(Box::new(pool))
        }
        PoolType::UniswapV3 => {
            let pool = fetch_v3_pool(
                provider,
                pool_address,
                block_number,
                token_registry,
                multicall_address,
            )
            .await?;
            Ok(Box::new(pool))
        }
        PoolType::ERC4626(pool_type) => {
            let pool = fetch_erc4626_pool(
                provider,
                pool_type,
                pool_address,
                block_number,
                token_registry,
            )
            .await?;
            Ok(pool)
        }
    }
}

pub async fn fetch_and_display_pool_info<P: Provider + Send + Sync>(
    provider: &Arc<P>,
    pool_addresses: &Vec<String>,
    block_number: BlockNumberOrTag,
    token_registry: &Arc<RwLock<TokenRegistry>>,
    pool_registry: &Arc<PoolRegistry>,
    // path_registry: &Arc<PathRegistry>,
    wait_time_for_startup: u64,
    multicall_address: Address,
) -> Result<()> {
    info!(
        "Starting pool fetch at block: {}",
        block_number.as_number().unwrap()
    );
    let mut pool_types_present = HashSet::new();
    for (i, pool_address) in pool_addresses.iter().enumerate() {
        info!("\nFetching pool information for address: {}", pool_address);

        // Parse the address
        let address: Address = pool_address.parse::<Address>()?;

        // Identify pool type
        let pool_type = identify_pool_type(provider, address).await?;
        info!("Pool type: {:?}", pool_type);

        // Fetch the pool
        match pool_type {
            PoolType::UniswapV2 => {
                let pool = fetch_v2_pool(
                    provider,
                    address,
                    BlockId::Number(block_number),
                    token_registry,
                    multicall_address,
                )
                .await?;
                pool_registry.add_pool(Box::new(pool.clone())).await;
                // path_registry.add_pool(&pool).await;
                pool_types_present.insert(PoolType::UniswapV2);
            }
            PoolType::UniswapV3 => {
                let pool = fetch_v3_pool(
                    provider,
                    address,
                    BlockId::Number(block_number),
                    token_registry,
                    multicall_address,
                )
                .await?;
                pool_registry.add_pool(Box::new(pool.clone())).await;
                // path_registry.add_pool(&pool).await;
                pool_types_present.insert(PoolType::UniswapV3);
            }
            PoolType::ERC4626(pool_type) => {
                let pool = fetch_erc4626_pool(
                    provider,
                    pool_type,
                    address,
                    BlockId::Number(block_number),
                    token_registry,
                )
                .await?;
                pool_registry.add_pool(pool.clone_box()).await;
                // path_registry.add_pool(&*pool).await;
                pool_types_present.insert(PoolType::ERC4626(pool_type));
            }
        };

        // Add delay between pools to respect rate limits
        if i < pool_addresses.len() - 1 {
            tokio::time::sleep(tokio::time::Duration::from_millis(wait_time_for_startup)).await;
        }
    }
    // Set last processed block
    pool_registry
        .set_last_processed_block(block_number.as_number().unwrap())
        .await;

    for pool_type in pool_types_present {
        pool_registry.add_topics(pool_type.topics()).await;
        pool_registry
            .add_profitable_topics(pool_type.profitable_topics())
            .await;
    }

    Ok(())
}
