use alloy::eips::BlockNumberOrTag;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, MULTICALL3_ADDRESS};
use alloy::rpc::client::RpcClient;
use alloy::transports::http::Http;
use alloy::transports::layers::FallbackLayer;
use anyhow::Result;
use evm_arb_bot::blockchain::pool_fetcher::fetch_and_display_pool_info;

use clap::Parser;
use env_logger::Env;
use evm_arb_bot::api::create_router;
use evm_arb_bot::blockchain::{
    EventQueue, PoolUpdaterLatestBlock, PoolUpdaterLatestBlockWs, WebsocketListener,
};
use evm_arb_bot::core::{proccessor::Proccessor, Database};

use evm_arb_bot::models::pool::multichain_registry::MultichainPoolRegistry;
use evm_arb_bot::models::pool::PoolRegistry;
use evm_arb_bot::models::token::{MultichainTokenRegistry, TokenRegistry};
use evm_arb_bot::utils::config::AppConfig;
use log::{error, info, LevelFilter};
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use url::Url;

// Example pool addresses

async fn initialize_chain(
    chain_config: evm_arb_bot::utils::config::ChainConfigs,
    multichain_pool_registry: Arc<MultichainPoolRegistry>,
    multichain_token_registry: Arc<MultichainTokenRegistry>,
    db: Option<Database>,
    should_load_snapshot_pool: bool,
) -> Result<(), anyhow::Error> {
    info!("Initializing chain...");

    // 1. Setup RPC provider
    let rpc_len = chain_config.rpc_urls.len();
    let fallback_layer =
        FallbackLayer::default().with_active_transport_count(NonZeroUsize::new(rpc_len).unwrap());

    let transports = chain_config
        .rpc_urls
        .iter()
        .map(|url| Http::new(Url::parse(url).unwrap()))
        .collect::<Vec<_>>();

    let transport = ServiceBuilder::new()
        .layer(fallback_layer)
        .service(transports);
    let client = RpcClient::builder().transport(transport, false);
    let provider = ProviderBuilder::new().connect_client(client.clone());
    let provider = Arc::new(provider);
    let chain_id = provider.get_chain_id().await.unwrap();

    info!("Connected to chain with ID: {}", chain_id);

    // 2. Create registries for this chain
    let pool_registry = Arc::new(PoolRegistry::new(chain_id));
    let token_registry = Arc::new(RwLock::new(TokenRegistry::new(chain_id)));

    // 3. Add to multichain registries
    multichain_pool_registry
        .add_pool_registry(chain_id, pool_registry.clone())
        .await;
    multichain_token_registry
        .add_token_registry(chain_id, token_registry.clone())
        .await;

    // 4. Load pools from database if available and if load_snapshot is enabled
    if should_load_snapshot_pool {
        if let Some(ref db) = db {
            info!("Loading pools from database for chain {}...", chain_id);
            if let Err(e) = pool_registry.load_from_db(db).await {
                error!(
                    "Error loading pools from database for chain {}: {}",
                    chain_id, e
                );
            }
            if let Err(e) = token_registry.write().await.load_from_db(db).await {
                error!(
                    "Error loading tokens from database for chain {}: {}",
                    chain_id, e
                );
            }
        }
    } else {
        info!(
            "Snapshot loading disabled for chain {}, starting with empty pool registry",
            chain_id
        );
        // 5. Initialize block number
        let last_processed_block = pool_registry.get_last_processed_block().await;
        let start_block = if should_load_snapshot_pool && last_processed_block > 0 {
            // Only use last_processed_block if loading snapshots is enabled
            last_processed_block
        } else {
            provider.get_block_number().await?
        };
        info!("Starting block for chain {}: {}", chain_id, start_block);

        // 6. Fetch additional pool information if needed
        info!("Fetching pool information from chain {}...", chain_id);
        let custom_multicall_address =
            if let Some(addr) = chain_config.custom_multicall_address.as_ref() {
                addr.parse::<Address>().unwrap()
            } else {
                MULTICALL3_ADDRESS
            };
        fetch_and_display_pool_info(
            &provider,
            &chain_config
                .pools
                .iter()
                .map(|p| p.address.clone())
                .collect(),
            BlockNumberOrTag::Number(start_block),
            &token_registry,
            &pool_registry,
            chain_config.wait_time_for_startup,
            custom_multicall_address,
        )
        .await?;

        // 7. Save pool registry to database
        if let Some(db) = db {
            info!(
                "Saving pool registry to database for chain {} at block: {}",
                chain_id,
                pool_registry.get_last_processed_block().await
            );
            pool_registry.save_to_db(&db).await?;
            token_registry.write().await.save_to_db(&db).await?;
        }
    }

    // 8. Start pool updater
    if chain_config.use_websocket {
        info!(
            "Starting pool updater with websocket for chain {}",
            chain_id
        );
        let event_queue = EventQueue::new(1000, 1000);

        for url in chain_config.websocket_urls {
            let event_sender = event_queue.get_sender().clone();
            let pool_registry_clone = pool_registry.clone();
            let chain_id_clone = chain_id;
            tokio::spawn(async move {
                let ws = WebsocketListener::new(
                    url,
                    pool_registry_clone.get_all_addresses().await,
                    event_sender,
                    pool_registry_clone.get_topics().await.clone(),
                );
                if let Err(e) = ws.start().await {
                    error!(
                        "Websocket listener error for chain {}: {}",
                        chain_id_clone, e
                    );
                }
            });
        }

        let mut pool_updater = PoolUpdaterLatestBlockWs::new(
            Arc::clone(&provider),
            event_queue,
            pool_registry.clone(),
            chain_config.max_blocks_per_batch,
        )
        .await;

        let chain_id_clone = chain_id;
        tokio::spawn(async move {
            if let Err(e) = pool_updater.start().await {
                error!("Pool updater error for chain {}: {}", chain_id_clone, e);
            }
        });
    } else {
        info!(
            "Starting pool updater with latest block for chain {}",
            chain_id
        );
        let mut pool_updater = PoolUpdaterLatestBlock::new(
            Arc::clone(&provider),
            pool_registry.clone(),
            pool_registry.get_last_processed_block().await,
            chain_config.max_blocks_per_batch,
        )
        .await;

        let chain_id_clone = chain_id;
        tokio::spawn(async move {
            if let Err(e) = pool_updater.start().await {
                error!("Pool updater error for chain {}: {}", chain_id_clone, e);
            }
        });
    }

    info!("Chain {} initialized successfully!", chain_id);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse command line arguments and setup logging
    let args = Args::parse();
    let log_level = match args.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

    // 2. Load configuration
    let config = AppConfig::load()?;

    // 3. Initialize databases
    // Initialize local database if configured
    let db = if let Some(db_path) = &config.database.db_path {
        info!("Initializing local database with name: {}", db_path);
        Some(Database::new(db_path)?)
    } else {
        None
    };

    let multichain_pool_registry = Arc::new(MultichainPoolRegistry::new());
    let multichain_token_registry = Arc::new(MultichainTokenRegistry::new());

    // Initialize all chains concurrently for faster startup
    let mut chain_handles = Vec::new();

    for (_chain_index, chain_config) in config.chain_configs.into_iter().enumerate() {
        let first_rpc = chain_config.rpc_urls.first().unwrap().clone();
        let multichain_pool_registry = multichain_pool_registry.clone();
        let multichain_token_registry = multichain_token_registry.clone();
        let db = db.clone();
        let should_load_snapshot_pool = config.database.load_snapshot_pool.unwrap_or(false);

        let handle = tokio::spawn(async move {
            let result = initialize_chain(
                chain_config,
                multichain_pool_registry,
                multichain_token_registry,
                db,
                should_load_snapshot_pool,
            )
            .await;
            (first_rpc, result)
        });

        chain_handles.push(handle);
    }

    // Wait for all chains to initialize
    info!("Waiting for all chains to initialize...");
    for handle in chain_handles {
        match handle.await? {
            (first_rpc, Ok(())) => {
                info!("Chain {} initialized successfully", first_rpc);
            }
            (first_rpc, Err(e)) => {
                error!("Chain {} initialization failed: {}", first_rpc, e);
                return Err(e.into());
            }
        }
    }
    info!("All chains initialized successfully!");

    // Create processor with multichain registries
    let processor = Arc::new(Proccessor::new(
        multichain_pool_registry.clone(),
        multichain_token_registry.clone(),
    ));

    // Start API server
    let app = create_router(processor);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("Starting API server on {}", addr);

    let _server_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Keep main thread alive with periodic database snapshot before exit
    let running = Arc::new(AtomicBool::new(true));

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
