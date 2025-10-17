use alloy::primitives::{utils::parse_units, Address, U256};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use log::info;
use std::sync::Arc;
use std::time::Instant;

use crate::{
    api::models::{
        BatchQuoteRequest, BatchQuoteRequestWithPool, BatchQuoteResponse, HealthResponse,
        NetworksResponse, PoolsResponse, QuoteRequestWithPool, QuoteResponse, TokenInfo,
        TokensResponse,
    },
    core::proccessor::QuoteType,
};
use crate::{
    api::models::{BatchQuoteRequestWithPools, BatchQuoteResponseWithSteps},
    core::proccessor::Proccessor,
    Token,
};

// Helper function to parse token address from optional string
fn parse_token_address(token_str: Option<String>) -> Result<Address, StatusCode> {
    token_str
        .ok_or(StatusCode::BAD_REQUEST)?
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)
}

// Helper function to validate token input and return appropriate error
fn validate_token_input(
    token_in: &Option<String>,
    token_out: &Option<String>,
) -> Result<(), QuoteResponse> {
    match (token_in, token_out) {
        (Some(_), Some(_)) => Err(QuoteResponse::error(
            "Only one of token_in or token_out should be provided".to_string(),
        )),
        (None, None) => Err(QuoteResponse::error(
            "Either token_in or token_out is required".to_string(),
        )),
        _ => Ok(()),
    }
}

// Helper function to validate batch token input and return appropriate error
fn validate_batch_token_input(
    token_in: &Option<String>,
    token_out: &Option<String>,
) -> Result<(), BatchQuoteResponse> {
    match (token_in, token_out) {
        (None, None) => Err(BatchQuoteResponse::error(
            "Either token_in or token_out is required".to_string(),
        )),
        _ => Ok(()),
    }
}

pub async fn health_check() -> Json<HealthResponse> {
    let start = Instant::now();
    let result = Json(HealthResponse {
        status: "ok".to_string(),
        message: "EVM Arbitrage Bot API is running".to_string(),
    });
    info!("GET /health completed in {:?}", start.elapsed());
    result
}

pub async fn quote_amount_in_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequestWithPool>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let amount_out = request
        .amount
        .parse::<U256>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        processor
            .quote_amount_in_token_in_raw(request.network_id, pool_address, token_in, amount_out)
            .await
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        processor
            .quote_amount_in_token_out_raw(request.network_id, pool_address, token_out, amount_out)
            .await
    };

    let response = match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    };
    info!(
        "POST /quote/amount-in/raw completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn quote_amount_in_token(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequestWithPool>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let result = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        processor
            .quote_amount_in_token_in(request.network_id, pool_address, token_in, request.amount)
            .await
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        processor
            .quote_amount_in_token_out(request.network_id, pool_address, token_out, request.amount)
            .await
    };

    let response = match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    };
    info!(
        "POST /quote/amount-in/token completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn quote_amount_out_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequestWithPool>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let amount_in = request
        .amount
        .parse::<U256>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        processor
            .quote_amount_out_token_in_raw(request.network_id, pool_address, token_in, amount_in)
            .await
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        processor
            .quote_amount_out_token_out_raw(request.network_id, pool_address, token_out, amount_in)
            .await
    };

    let response = match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    };
    info!(
        "POST /quote/amount-out/raw completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn quote_amount_out_token(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequestWithPool>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let result = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        processor
            .quote_amount_out_token_in(request.network_id, pool_address, token_in, request.amount)
            .await
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        processor
            .quote_amount_out_token_out(request.network_id, pool_address, token_out, request.amount)
            .await
    };

    let response = match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    };
    info!(
        "POST /quote/amount-out/token completed in {:?}",
        start.elapsed()
    );
    response
}

// Batch quote handlers
pub async fn batch_quote_amount_in_raw_with_pool(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequestWithPool>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    // Parse all amounts as decimal
    let amounts: Result<Vec<U256>, StatusCode> = request
        .amounts
        .iter()
        .map(|amount_str| {
            amount_str
                .parse::<U256>()
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .collect();

    let amounts = amounts?;

    let results = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        let mut results = Vec::new();
        for amount in amounts {
            match processor
                .quote_amount_in_token_in_raw(request.network_id, pool_address, token_in, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        let mut results = Vec::new();
        for amount in amounts {
            match processor
                .quote_amount_in_token_out_raw(request.network_id, pool_address, token_out, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    };

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-in/raw completed in {:?}",
        start.elapsed()
    );
    response
}

// Batch quote handlers
pub async fn batch_quote_amount_in_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequest>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    // Parse all amounts as decimal
    let amounts: Result<Vec<U256>, StatusCode> = request
        .amounts
        .iter()
        .map(|amount_str| {
            amount_str
                .parse::<U256>()
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .collect();

    let amounts = amounts?;

    let token_in = parse_token_address(request.token_in)?;
    let token_out = parse_token_address(request.token_out)?;
    let paths = processor
        .pool_registry()
        .get_pool_registry(request.network_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .get_all_path_from_token_to_token(token_in, token_out, 3)
        .await;

    let mut results = Vec::new();
    for amount in amounts {
        let mut best_result = U256::MAX;
        for path in &paths {
            match processor
                .quote_amount_token_with_path_raw(
                    request.network_id,
                    &path,
                    amount,
                    &QuoteType::ExactOut,
                    token_in,
                    token_out,
                )
                .await
            {
                Ok(result) => {
                    let input_amount = result
                        .input
                        .amount
                        .parse::<U256>()
                        .map_err(|_| StatusCode::BAD_REQUEST)?;
                    if input_amount < best_result {
                        best_result = input_amount;
                    }
                }
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results.push(best_result);
    }

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-in/path/raw completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn batch_quote_amount_in_token_with_pool(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequestWithPool>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let results = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        let mut results = Vec::new();
        for amount in request.amounts {
            match processor
                .quote_amount_in_token_in(request.network_id, pool_address, token_in, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        let mut results = Vec::new();
        for amount in request.amounts {
            match processor
                .quote_amount_in_token_out(request.network_id, pool_address, token_out, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    };

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-in/token completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn batch_quote_amount_out_raw_with_pool(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequestWithPool>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    // Parse all amounts as decimal
    let amounts: Result<Vec<U256>, StatusCode> = request
        .amounts
        .iter()
        .map(|amount_str| {
            amount_str
                .parse::<U256>()
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .collect();

    let amounts = amounts?;

    let results = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        let mut results = Vec::new();
        for amount in amounts {
            match processor
                .quote_amount_out_token_in_raw(request.network_id, pool_address, token_in, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        let mut results = Vec::new();
        for amount in amounts {
            match processor
                .quote_amount_out_token_out_raw(request.network_id, pool_address, token_out, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    };

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-out/raw completed in {:?}",
        start.elapsed()
    );
    response
}

// Batch quote handlers
pub async fn batch_quote_amount_out_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequest>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    // Parse all amounts as decimal
    let amounts: Result<Vec<U256>, StatusCode> = request
        .amounts
        .iter()
        .map(|amount_str| {
            amount_str
                .parse::<U256>()
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .collect();

    let amounts = amounts?;

    let token_in = parse_token_address(request.token_in)?;
    let token_out = parse_token_address(request.token_out)?;
    let paths = processor
        .pool_registry()
        .get_pool_registry(request.network_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .get_all_path_from_token_to_token(token_in, token_out, 3)
        .await;

    let mut results = Vec::new();
    for amount in amounts {
        let mut best_result = U256::ZERO;
        for path in &paths {
            match processor
                .quote_amount_token_with_path_raw(
                    request.network_id,
                    &path,
                    amount,
                    &QuoteType::ExactIn,
                    token_in,
                    token_out,
                )
                .await
            {
                Ok(result) => {
                    let output_amount = result
                        .output
                        .amount
                        .parse::<U256>()
                        .map_err(|_| StatusCode::BAD_REQUEST)?;
                    if output_amount > best_result {
                        best_result = output_amount;
                    }
                }
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results.push(best_result);
    }

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-out/path/raw completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn batch_quote_amount_out_token_with_pools(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequestWithPools>,
) -> Result<Json<BatchQuoteResponseWithSteps>, StatusCode> {
    let start = Instant::now();
    // Parse all amounts as decimal
    let first_pool = request.pools.first().unwrap();
    let token_in = parse_token_address(Some(first_pool.token_in.clone()))?;
    let token_in_decimals = processor
        .token_registry()
        .get_token(first_pool.network_id, token_in)
        .await
        .ok_or(StatusCode::NOT_FOUND)?
        .decimals;
    let amounts: Result<Vec<U256>, StatusCode> = request
        .amounts
        .iter()
        .map(|amount_str| {
            parse_units(amount_str, token_in_decimals)
                .map(Into::into)
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .collect();

    let mut amounts = amounts?;
    let mut last_token = None;
    let mut steps = Vec::new();
    let mut step_tokens = Vec::new();
    let mut step_decimals = Vec::new();
    for pool in request.pools {
        let pool_address = pool
            .pool_address
            .parse::<Address>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        let pool_token_in = parse_token_address(Some(pool.token_in))?;
        let pool_token_in_data = processor
            .token_registry()
            .get_token(pool.network_id, pool_token_in)
            .await
            .ok_or(StatusCode::NOT_FOUND)?;
        if last_token.is_some() {
            let last_token_obj: Token = last_token.unwrap();
            if last_token_obj.decimals != pool_token_in_data.decimals {
                // Convert last token to pool token in decimals
                let this_token_mult: U256 = parse_units("1", pool_token_in_data.decimals)
                    .map(Into::into)
                    .unwrap();
                let last_token_mult: U256 = parse_units("1", last_token_obj.decimals)
                    .map(Into::into)
                    .unwrap();
                amounts = amounts
                    .iter()
                    .map(|amount| *amount * this_token_mult / last_token_mult)
                    .collect();
            }
        }

        let mut step_amounts = Vec::new();
        for amount in amounts {
            match processor
                .quote_amount_out_token_in_raw(pool.network_id, pool_address, pool_token_in, amount)
                .await
            {
                Ok(result) => {
                    step_amounts.push(result);
                }
                Err(e) => return Ok(Json(BatchQuoteResponseWithSteps::error(e.to_string()))),
            }
        }
        amounts = step_amounts;
        let (token0, token1) = processor
            .pool_registry()
            .get_pool_registry(pool.network_id)
            .await
            .ok_or(StatusCode::NOT_FOUND)?
            .get_pool(&pool_address)
            .await
            .ok_or(StatusCode::NOT_FOUND)?
            .read()
            .await
            .tokens();
        let token_out = if pool_token_in == token0 {
            token1
        } else {
            token0
        };
        last_token = processor
            .token_registry()
            .get_token(pool.network_id, token_out)
            .await
            .ok_or(StatusCode::NOT_FOUND)
            .ok();

        step_tokens.push(last_token.as_ref().unwrap().symbol.clone());
        step_decimals.push(last_token.as_ref().unwrap().decimals.clone());
        steps.push(amounts.clone());
    }

    let response = Ok(Json(BatchQuoteResponseWithSteps::success(
        amounts,
        steps,
        step_tokens,
        step_decimals,
    )));
    info!(
        "POST /quote/batch/amount-out/pools/raw completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn batch_quote_amount_out_token_with_pool(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<BatchQuoteRequestWithPool>,
) -> Result<Json<BatchQuoteResponse>, StatusCode> {
    let start = Instant::now();
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_batch_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let results = if let Some(token_in_str) = request.token_in {
        let token_in = parse_token_address(Some(token_in_str))?;
        let mut results = Vec::new();
        for amount in request.amounts {
            match processor
                .quote_amount_out_token_in(request.network_id, pool_address, token_in, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    } else {
        let token_out_str = request.token_out.unwrap(); // Safe because we validated above
        let token_out = parse_token_address(Some(token_out_str))?;
        let mut results = Vec::new();
        for amount in request.amounts {
            match processor
                .quote_amount_out_token_out(request.network_id, pool_address, token_out, amount)
                .await
            {
                Ok(result) => results.push(result),
                Err(e) => return Ok(Json(BatchQuoteResponse::error(e.to_string()))),
            }
        }
        results
    };

    let response = Ok(Json(BatchQuoteResponse::success(results)));
    info!(
        "POST /quote/batch/amount-out/token completed in {:?}",
        start.elapsed()
    );
    response
}

pub async fn get_networks(
    State(processor): State<Arc<Proccessor>>,
) -> Result<Json<NetworksResponse>, StatusCode> {
    let start = Instant::now();
    let networks = processor.pool_registry().get_all_network_ids().await;

    let response = Ok(Json(NetworksResponse {
        networks: networks.clone(),
        total_networks: networks.len(),
    }));
    info!("GET /networks completed in {:?}", start.elapsed());
    response
}

pub async fn get_pools(
    State(processor): State<Arc<Proccessor>>,
    Path(network_id): Path<u64>,
) -> Result<Json<PoolsResponse>, StatusCode> {
    let start = Instant::now();
    let pool_registry = processor
        .pool_registry()
        .get_pool_registry(network_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let pool_addresses = pool_registry.get_all_addresses().await;
    let pool_strings: Vec<String> = pool_addresses
        .iter()
        .map(|addr| format!("{:?}", addr))
        .collect();

    let response = Ok(Json(PoolsResponse {
        network_id,
        pools: pool_strings.clone(),
        total_pools: pool_strings.len(),
    }));
    info!(
        "GET /networks/{}/pools completed in {:?}",
        network_id,
        start.elapsed()
    );
    response
}

pub async fn get_tokens(
    State(processor): State<Arc<Proccessor>>,
    Path(network_id): Path<u64>,
) -> Result<Json<TokensResponse>, StatusCode> {
    let start = Instant::now();
    let token_registry = processor
        .token_registry()
        .get_token_registry(network_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let registry_guard = token_registry.read().await;
    let tokens = registry_guard.get_all_tokens();
    let token_infos: Vec<TokenInfo> = tokens
        .iter()
        .map(|token| TokenInfo {
            address: format!("{:?}", token.address),
            symbol: token.symbol.clone(),
            name: token.name.clone(),
            decimals: token.decimals,
        })
        .collect();

    let response = Ok(Json(TokensResponse {
        network_id,
        tokens: token_infos.clone(),
        total_tokens: token_infos.len(),
    }));
    info!(
        "GET /networks/{}/tokens completed in {:?}",
        network_id,
        start.elapsed()
    );
    response
}
