use alloy::primitives::{Address, U256};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

use crate::api::models::{
    HealthResponse, NetworksResponse, PoolsResponse, QuoteRequest, QuoteResponse, TokenInfo,
    TokensResponse,
};
use crate::core::proccessor::Proccessor;

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

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "EVM Arbitrage Bot API is running".to_string(),
    })
}

pub async fn quote_amount_in_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let amount_out = U256::from_str_radix(request.amount.trim_start_matches("0x"), 16)
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

    match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    }
}

pub async fn quote_amount_in_token(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, StatusCode> {
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

    match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    }
}

pub async fn quote_amount_out_raw(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, StatusCode> {
    let pool_address = request
        .pool
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate token input
    if let Err(error_response) = validate_token_input(&request.token_in, &request.token_out) {
        return Ok(Json(error_response));
    }

    let amount_in = U256::from_str_radix(request.amount.trim_start_matches("0x"), 16)
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

    match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    }
}

pub async fn quote_amount_out_token(
    State(processor): State<Arc<Proccessor>>,
    Json(request): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, StatusCode> {
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

    match result {
        Ok(amount) => Ok(Json(QuoteResponse::success(amount))),
        Err(e) => Ok(Json(QuoteResponse::error(e.to_string()))),
    }
}

pub async fn get_networks(
    State(processor): State<Arc<Proccessor>>,
) -> Result<Json<NetworksResponse>, StatusCode> {
    let networks = processor.pool_registry().get_all_network_ids().await;

    Ok(Json(NetworksResponse {
        networks: networks.clone(),
        total_networks: networks.len(),
    }))
}

pub async fn get_pools(
    State(processor): State<Arc<Proccessor>>,
    Path(network_id): Path<u64>,
) -> Result<Json<PoolsResponse>, StatusCode> {
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

    Ok(Json(PoolsResponse {
        network_id,
        pools: pool_strings.clone(),
        total_pools: pool_strings.len(),
    }))
}

pub async fn get_tokens(
    State(processor): State<Arc<Proccessor>>,
    Path(network_id): Path<u64>,
) -> Result<Json<TokensResponse>, StatusCode> {
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

    Ok(Json(TokensResponse {
        network_id,
        tokens: token_infos.clone(),
        total_tokens: token_infos.len(),
    }))
}
