use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("Pool error: {0}")]
    PoolError(String),

    #[error("Simulation error: {0}")]
    SimulationError(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Provider error: {0}")]
    ProviderError(String),
}

pub type Result<T> = std::result::Result<T, ArbitrageError>;
