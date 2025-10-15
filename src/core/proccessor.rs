use alloy::primitives::{Address, U256};
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{
    pool::multichain_registry::MultichainPoolRegistry,
    token::multichain_registry::MultichainTokenRegistry,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum QuoteType {
    #[serde(rename = "exact_in")]
    ExactIn,
    #[serde(rename = "exact_out")]
    ExactOut,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteData {
    pub input: InputToken,
    pub output: OutputToken,
    pub route: Vec<RouteStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputToken {
    pub amount: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputToken {
    pub amount: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteStep {
    pub address: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
}

pub struct Proccessor {
    pool_registry: Arc<MultichainPoolRegistry>,
    token_registry: Arc<MultichainTokenRegistry>,
}

impl Proccessor {
    pub fn new(
        pool_registry: Arc<MultichainPoolRegistry>,
        token_registry: Arc<MultichainTokenRegistry>,
    ) -> Self {
        Self {
            pool_registry,
            token_registry,
        }
    }

    pub fn pool_registry(&self) -> &Arc<MultichainPoolRegistry> {
        &self.pool_registry
    }

    pub fn token_registry(&self) -> &Arc<MultichainTokenRegistry> {
        &self.token_registry
    }

    pub async fn quote_amount_in_token_in_raw(
        &self,
        network_id: u64,
        pool: Address,
        token_in: Address,
        amount_out: U256,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let (token0, token1) = pool.read().await.tokens();
        let token_out = if token_in == token0 { token1 } else { token0 };
        let amount_in = pool.read().await.calculate_input(&token_out, amount_out)?;
        Ok(amount_in)
    }

    pub async fn quote_amount_token_with_path_raw(
        &self,
        network_id: u64,
        path: &[Address],
        amount: U256,
        quote_type: &QuoteType,
        token_in: Address,
        token_out: Address,
    ) -> Result<QuoteData> {
        let mut current_amount = amount;
        let mut path_steps = Vec::new();

        // For exact output, we need to process the path in reverse
        let path_to_process = match quote_type {
            QuoteType::ExactIn => path.iter().collect::<Vec<_>>(),
            QuoteType::ExactOut => path.iter().rev().collect::<Vec<_>>(),
        };

        let mut current_token = match quote_type {
            QuoteType::ExactIn => token_in,   // Start with input token
            QuoteType::ExactOut => token_out, // Start with output token for reverse calculation
        };
        for &pool_address in path_to_process.iter() {
            let pool_arc = self
                .pool_registry
                .get_pool_registry(network_id)
                .await
                .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
                .get_pool(&pool_address)
                .await
                .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;

            let (token0, token1) = pool_arc.read().await.tokens();

            // Determine the correct token pair for this step
            let (step_token_in, step_token_out) = match quote_type {
                QuoteType::ExactIn => {
                    // For exact input, current_token is the input token
                    if current_token == token0 {
                        (token0, token1)
                    } else if current_token == token1 {
                        (token1, token0)
                    } else {
                        return Err(anyhow!(
                            "Token {:?} not found in pool {:?} with tokens {:?}, {:?}",
                            current_token,
                            pool_address,
                            token0,
                            token1
                        ));
                    }
                }
                QuoteType::ExactOut => {
                    // For exact output (reverse path), current_token is the output token
                    if current_token == token0 {
                        (token1, token0) // Reverse: output token becomes input for next step
                    } else if current_token == token1 {
                        (token0, token1) // Reverse: output token becomes input for next step
                    } else {
                        return Err(anyhow!(
                            "Token {:?} not found in pool {:?} with tokens {:?}, {:?}",
                            current_token,
                            pool_address,
                            token0,
                            token1
                        ));
                    }
                }
            };

            // Calculate the quote for this step
            let step_amount_out = match quote_type {
                QuoteType::ExactIn => {
                    let output = pool_arc
                        .read()
                        .await
                        .calculate_output(&step_token_in, current_amount)?;

                    output
                }
                QuoteType::ExactOut => {
                    // For exact out (reverse path), calculate input required for desired output
                    let input_amount = pool_arc
                        .read()
                        .await
                        .calculate_input(&step_token_out, current_amount)?;

                    input_amount
                }
            };

            let token_registry = self
                .token_registry
                .get_token_registry(network_id)
                .await
                .ok_or_else(|| anyhow::anyhow!("Token registry not found"))?;
            let token_registry_guard = token_registry.read().await;
            // Get token info from token registry
            let token_in_info = {
                let token_registry = token_registry_guard.get_token(step_token_in);
                if let Some(token) = token_registry {
                    token.symbol.clone()
                } else {
                    "UNKNOWN".to_string()
                }
            };

            let token_out_info = {
                let token_registry = token_registry_guard.get_token(step_token_out);
                if let Some(token) = token_registry {
                    token.symbol.clone()
                } else {
                    "UNKNOWN".to_string()
                }
            };
            drop(token_registry_guard);

            // Create route step
            let route_step = RouteStep {
                address: format!("{:?}", pool_address),
                token_in: token_in_info,
                token_out: token_out_info,
                amount_in: match quote_type {
                    QuoteType::ExactIn => current_amount.to_string(),
                    QuoteType::ExactOut => step_amount_out.to_string(), // For reverse path, step_amount_out is the input to this step
                },
                amount_out: match quote_type {
                    QuoteType::ExactIn => step_amount_out.to_string(),
                    QuoteType::ExactOut => current_amount.to_string(), // For reverse path, current_amount is the output from this step
                },
            };

            path_steps.push(route_step);

            // Update current amount and token for next iteration
            match quote_type {
                QuoteType::ExactIn => {
                    current_amount = step_amount_out;
                    current_token = step_token_out;
                }
                QuoteType::ExactOut => {
                    current_amount = step_amount_out;
                    current_token = step_token_in; // For reverse path, step_token_in is the next token
                }
            }
        }

        // Verify we end up with the correct final token
        let expected_final_token = match quote_type {
            QuoteType::ExactIn => token_out,
            QuoteType::ExactOut => token_in,
        };

        if current_token != expected_final_token {
            return Err(anyhow!(
                "Path does not end with expected token. Expected: {:?}, Got: {:?}",
                expected_final_token,
                current_token
            ));
        }

        // Create input and output token info
        let (input_amount, output_amount) = match quote_type {
            QuoteType::ExactIn => (amount, current_amount),
            QuoteType::ExactOut => (current_amount, amount), // For exact out, we calculated the required input
        };

        let input_token = InputToken {
            amount: input_amount.to_string(),
            token: format!("{:?}", token_in),
        };

        let output_token = OutputToken {
            amount: output_amount.to_string(),
            token: format!("{:?}", token_out),
        };

        // For exact output, reverse the path steps to show them in correct order
        let final_path_steps = match quote_type {
            QuoteType::ExactIn => path_steps,
            QuoteType::ExactOut => path_steps.into_iter().rev().collect(),
        };

        Ok(QuoteData {
            input: input_token,
            output: output_token,
            route: final_path_steps,
        })
    }

    pub async fn quote_amount_in_token_in(
        &self,
        network_id: u64,
        pool: Address,
        token_in: Address,
        amount_out_str: String,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let (token0, token1) = pool.read().await.tokens();
        let token_out = if token_in == token0 { token1 } else { token0 };
        let amount_out = self
            .token_registry
            .get_token_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Token registry not found"))?
            .read()
            .await
            .get_token(token_out)
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?
            .to_raw_amount(&amount_out_str)?;
        let amount_in = pool.read().await.calculate_input(&token_out, amount_out)?;
        Ok(amount_in)
    }

    pub async fn quote_amount_in_token_out_raw(
        &self,
        network_id: u64,
        pool: Address,
        token_out: Address,
        amount_out: U256,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let amount_in = pool.read().await.calculate_input(&token_out, amount_out)?;
        Ok(amount_in)
    }

    pub async fn quote_amount_in_token_out(
        &self,
        network_id: u64,
        pool: Address,
        token_out: Address,
        amount_out_str: String,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let amount_out = self
            .token_registry
            .get_token_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Token registry not found"))?
            .read()
            .await
            .get_token(token_out)
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?
            .to_raw_amount(&amount_out_str)?;
        let amount_in = pool.read().await.calculate_input(&token_out, amount_out)?;
        Ok(amount_in)
    }

    pub async fn quote_amount_out_token_in(
        &self,
        network_id: u64,
        pool: Address,
        token_in: Address,
        amount_in_str: String,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let amount_in = self
            .token_registry
            .get_token_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Token registry not found"))?
            .read()
            .await
            .get_token(token_in)
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?
            .to_raw_amount(&amount_in_str)?;
        let amount_out = pool.read().await.calculate_output(&token_in, amount_in)?;
        Ok(amount_out)
    }

    pub async fn quote_amount_out_token_in_raw(
        &self,
        network_id: u64,
        pool: Address,
        token_in: Address,
        amount_in: U256,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let amount_out = pool.read().await.calculate_output(&token_in, amount_in)?;
        Ok(amount_out)
    }

    pub async fn quote_amount_out_token_out(
        &self,
        network_id: u64,
        pool: Address,
        token_out: Address,
        amount_in_str: String,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let (token0, token1) = pool.read().await.tokens();
        let token_in = if token_out == token0 { token1 } else { token0 };
        let amount_in = self
            .token_registry
            .get_token_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Token registry not found"))?
            .read()
            .await
            .get_token(token_in)
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?
            .to_raw_amount(&amount_in_str)?;
        let amount_out = pool.read().await.calculate_output(&token_in, amount_in)?;
        Ok(amount_out)
    }

    pub async fn quote_amount_out_token_out_raw(
        &self,
        network_id: u64,
        pool: Address,
        token_out: Address,
        amount_in: U256,
    ) -> Result<U256> {
        let pool = self
            .pool_registry
            .get_pool_registry(network_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool registry not found"))?
            .get_pool(&pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Pool not found"))?;
        let (token0, token1) = pool.read().await.tokens();
        let token_in = if token_out == token0 { token1 } else { token0 };
        let amount_out = pool.read().await.calculate_output(&token_in, amount_in)?;
        Ok(amount_out)
    }
}
