use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::sync::Arc;

use crate::models::{
    pool::multichain_registry::MultichainPoolRegistry,
    token::multichain_registry::MultichainTokenRegistry,
};

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
