use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequestWithPool {
    pub network_id: u64,
    pub pool: String,              // Address as string
    pub token_in: Option<String>,  // Address as string
    pub token_out: Option<String>, // Address as string
    pub amount: String,            // Amount as string (for token amounts) or hex (for raw amounts)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQuoteRequestWithPool {
    pub network_id: u64,
    pub pool: String,              // Address as string
    pub token_in: Option<String>,  // Address as string
    pub token_out: Option<String>, // Address as string
    pub amounts: Vec<String>, // Array of amounts as strings (for token amounts) or hex (for raw amounts)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQuoteRequest {
    pub network_id: u64,
    pub token_in: Option<String>,  // Address as string
    pub token_out: Option<String>, // Address as string
    pub amounts: Vec<String>, // Array of amounts as strings (for token amounts) or hex (for raw amounts)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolRequest {
    pub token_in: String,
    pub pool_address: String,
    pub network_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQuoteRequestWithPools {
    pub token_in: String,          // Address as string
    pub token_out: Option<String>, // Address as string
    pub amounts: Vec<String>, // Array of amounts as strings (for token amounts) or hex (for raw amounts)
    pub pools: Vec<PoolRequest>, // Array of pool addresses as strings
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub success: bool,
    pub result: Option<String>, // U256 as hex string
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQuoteResponse {
    pub success: bool,
    pub results: Option<Vec<String>>, // Array of U256 as hex strings
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQuoteResponseWithSteps {
    pub success: bool,
    pub results: Option<Vec<String>>, // Array of arrays of U256 as hex strings
    pub steps: Option<Vec<Vec<String>>>, // Array of arrays of U256 as hex strings
    pub step_tokens: Option<Vec<String>>, // Array of arrays of U256 as hex strings
    pub step_decimals: Option<Vec<u8>>, // Array of arrays of u8
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworksResponse {
    pub networks: Vec<u64>,
    pub total_networks: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsResponse {
    pub network_id: u64,
    pub pools: Vec<String>, // Pool addresses as strings
    pub total_pools: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensResponse {
    pub network_id: u64,
    pub tokens: Vec<TokenInfo>,
    pub total_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

impl QuoteResponse {
    pub fn success(result: U256) -> Self {
        Self {
            success: true,
            result: Some(result.to_string()),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            result: None,
            error: Some(error),
        }
    }
}

impl BatchQuoteResponseWithSteps {
    pub fn success(
        results: Vec<U256>,
        steps: Vec<Vec<U256>>,
        step_tokens: Vec<String>,
        step_decimals: Vec<u8>,
    ) -> Self {
        Self {
            success: true,
            results: Some(results.into_iter().map(|r| r.to_string()).collect()),
            steps: Some(
                steps
                    .into_iter()
                    .map(|s| s.into_iter().map(|r| r.to_string()).collect())
                    .collect(),
            ),
            step_tokens: Some(step_tokens),
            step_decimals: Some(step_decimals),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            results: None,
            steps: None,
            step_tokens: None,
            step_decimals: None,
            error: Some(error),
        }
    }
}

impl BatchQuoteResponse {
    pub fn success(results: Vec<U256>) -> Self {
        Self {
            success: true,
            results: Some(results.into_iter().map(|r| r.to_string()).collect()),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            results: None,
            error: Some(error),
        }
    }
}
