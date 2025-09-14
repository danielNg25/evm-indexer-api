pub mod api;
pub mod blockchain;
// pub mod config;
pub mod core;
pub mod models;
// pub mod services;
pub mod utils;

// Re-export commonly used types and functions
// pub use crate::core::graph::Graph;
// pub use crate::core::path_finder::PathFinder;
// pub use crate::core::simulator::Simulator;
// pub use crate::models::path::ArbPath;
pub use crate::models::pool::{PoolInterface, PoolType, UniswapV2Pool};
pub use crate::models::token::Token;
// pub use crate::services::pool_sync::PoolSyncService;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
