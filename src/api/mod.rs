use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::core::proccessor::Proccessor;

pub mod handlers;
pub mod models;

pub fn create_router(processor: Arc<Proccessor>) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/networks", get(handlers::get_networks))
        .route("/networks/:network_id/pools", get(handlers::get_pools))
        .route("/networks/:network_id/tokens", get(handlers::get_tokens))
        .route("/quote/amount-in/raw", post(handlers::quote_amount_in_raw))
        .route(
            "/quote/amount-in/token",
            post(handlers::quote_amount_in_token),
        )
        .route(
            "/quote/amount-out/raw",
            post(handlers::quote_amount_out_raw),
        )
        .route(
            "/quote/amount-out/token",
            post(handlers::quote_amount_out_token),
        )
        // Batch quote endpoints
        .route(
            "/quote/batch/amount-in/raw",
            post(handlers::batch_quote_amount_in_raw),
        )
        .route(
            "/quote/batch/amount-in/token",
            post(handlers::batch_quote_amount_in_token),
        )
        .route(
            "/quote/batch/amount-out/raw",
            post(handlers::batch_quote_amount_out_raw),
        )
        .route(
            "/quote/batch/amount-out/token",
            post(handlers::batch_quote_amount_out_token),
        )
        .with_state(processor)
}
