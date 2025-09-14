# EVM Arbitrage Bot

A high-performance arbitrage bot for Ethereum and EVM-compatible blockchains, built in Rust.

## Features

-   Supports Uniswap V2 and V3 compatible pools
-   Real-time monitoring of swap events using RPC
-   Efficient graph-based path finding for detecting arbitrage opportunities
-   Concurrent processing for high throughput
-   Strategy-based configuration for different trading approaches
-   Robust error handling and logging
-   MongoDB integration for data collection and analytics
-   Multi-network operation with centralized data storage
-   REST API for dashboard integration

## Prerequisites

-   Rust toolchain (1.60+)
-   JSON-RPC access to an EVM blockchain (Ethereum, Arbitrum, Polygon, etc.)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/evm-arbitrage-bot.git
cd evm-arbitrage-bot
```

2. Build the project:

```bash
cargo build --release
```

## Configuration

The bot uses a strategy-based configuration system. Create strategy files in the `configs/strategies` directory:

```toml
# configs/strategies/default.toml
rpc_url = "https://binance.llamarpc.com"
start_block = 48342105
max_blocks_per_batch = 10
pool_addresses = [
    "0x27c4c15Ba98dC84ABFf05aA4C6f9086337F07472",
    "0xF963Bd12A4653E82CA7B739e76EC4c67808AFF34"
]
log_level = "info"
output_dir = "./target"
db_path = "pools_db"         # Database name (stored in ./data folder)
snapshot_interval = 300      # Save pools to database every 5 minutes (300 seconds)
load_snapshot = true         # Load data from snapshot at startup (default: true)

[[profit_tokens]]
token = "0x27c4c15Ba98dC84ABFf05aA4C6f9086337F07472"
min_profit = "0.3"
max_amount = "10000"

[[profit_tokens]]
token = "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c"
min_profit = "0.0005"
max_amount = "20"
```

### Database Persistence

The bot now includes database support for persisting pool data, which provides the following benefits:

-   Faster startup times by loading cached pool data
-   Preserves pool states between application restarts
-   Reduces RPC call volume during initialization

To enable database persistence, add these options to your strategy configuration:

-   `db_path`: Name of the database (will be stored in the `./data` folder, which is created automatically)
-   `snapshot_interval`: Interval in seconds for automatic database snapshots (optional)
-   `load_snapshot`: Whether to load the snapshot at startup (defaults to true if not specified)

The database is automatically saved when the bot starts a graceful shutdown (CTRL+C).

### MongoDB Integration

The bot supports MongoDB for centralized data storage across multiple networks and instances. This enables:

-   Collection of arbitrage opportunities data from all running instances
-   Tracking of profitability across different networks
-   Building real-time dashboards and analytics

To enable MongoDB, add the following configuration to your strategy TOML file:

```toml
# Enable MongoDB indexing
use_mongodb = true

# MongoDB Configuration
[mongodb]
enable_indexing = true
uri = "mongodb://localhost:27017"
database = "arbitrage_bot"
# instance_id = "server-01"  # Optional: Unique instance identifier
```

The MongoDB configuration will be loaded from your strategy file, and the network ID will be automatically set from your strategy configuration. This makes it easier to manage different configurations for different networks or strategies.

## Usage

### Running the Bot

```bash
# Run with default strategy
cargo run

# Run with specific strategy
cargo run -- --strategy high_volume

# Run with increased log verbosity
cargo run -- --log-level debug
```

### Command Line Options

```
USAGE:
    evm-arb-bot [OPTIONS]

OPTIONS:
    -h, --help                       Print help information
    -l, --log-level <LOG_LEVEL>      Logging verbosity level [default: info]
    -s, --strategy <STRATEGY>        Strategy to use [default: default]
```

## Architecture

The bot is structured into the following components:

1. **Models** - Core data structures

    - `Token` - Token information and metadata
    - `Pool` - Uniswap V2/V3 pool implementations
    - `Path` - Arbitrage path representation
    - `ProfitToken` - Configuration for profit tokens

2. **Blockchain** - Blockchain interaction layer

    - `PoolFetcher` - Fetches and updates pool data
    - `PoolUpdater` - Maintains real-time pool state
    - `EventProcessor` - Processes blockchain events

3. **Core** - Business logic

    - `Simulator` - Simulates arbitrage opportunities
    - `PathFinder` - Finds profitable arbitrage paths
    - `Registry` - Manages pools, tokens, and paths

4. **Utils** - Supporting functionality
    - `Config` - Configuration management
    - `Metrics` - Performance monitoring
    - `Errors` - Error handling

## Performance Optimization

-   Use WebSocket connections for real-time event monitoring
-   Adjust the scan interval and batch sizes based on your infrastructure
-   Configure appropriate profit thresholds for your trading parameters

## TODO

-   [x] Implement database caching pools data for faster startup
-   [x] Add back latest block proccessing for network that not support pending block
-   [x] Add branch cut off instead of full simulation
-   [x] Implement websocket
-   [x] Implement submit to multiple rpc
-   [ ] Implement profit compare gas fee and bid profit
-   [ ] Implement profit token ranking, prioritize profit tokens
-   [x] Create a web dashboard for monitoring
-   [x] Add MongoDB integration for data collection
-   [ ] Implement backtesting functionality
-   [ ] Implement minimum amount cut off
-   [ ] RamsesV2 pool
-   [ ] Algebra pool
-   [ ] hook stable pool

# Issue

-   Fraxtal: revert too many, need to investigate reason (bot or opponent)
-   Kava: revert too many, need to investigate reason (bot or opponent)
-   CORE: revert too many, need to investigate reason (bot issue) => Bid gas problem
-   Hedera: They use hedera token service
-   Ink: Revert to many, execute at block n+2, some bot can execute at n+1
-   Avalanche: Bid gas problem

# Fix openssl issue

```sh
sudo apt install libssl-dev
sudo apt install pkg-config
```
