use alloy::primitives::Address;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::token::registry::TokenRegistry;

#[derive(Debug, Default)]
pub struct MultichainTokenRegistry {
    registries: Arc<RwLock<HashMap<u64, Arc<RwLock<TokenRegistry>>>>>,
}

impl MultichainTokenRegistry {
    pub fn new() -> Self {
        Self {
            registries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_token_registry(
        &self,
        network_id: u64,
        token_registry: Arc<RwLock<TokenRegistry>>,
    ) {
        let mut registries = self.registries.write().await;
        registries.insert(network_id, token_registry);
    }

    pub async fn get_token_registry(&self, network_id: u64) -> Option<Arc<RwLock<TokenRegistry>>> {
        let registries = self.registries.read().await;
        registries.get(&network_id).map(Arc::clone)
    }

    pub async fn get_all_network_ids(&self) -> Vec<u64> {
        let registries = self.registries.read().await;
        registries.keys().cloned().collect()
    }

    pub async fn remove_token_registry(
        &self,
        network_id: u64,
    ) -> Option<Arc<RwLock<TokenRegistry>>> {
        let mut registries = self.registries.write().await;
        registries.remove(&network_id)
    }

    pub async fn registry_count(&self) -> usize {
        let registries = self.registries.read().await;
        registries.len()
    }

    pub async fn total_tokens_across_networks(&self) -> usize {
        let registries = self.registries.read().await;
        let mut total = 0;
        for registry in registries.values() {
            total += registry.read().await.token_count();
        }
        total
    }

    pub async fn contains_token_registry(&self, network_id: u64) -> bool {
        let registries = self.registries.read().await;
        registries.contains_key(&network_id)
    }

    pub async fn contains_token(&self, network_id: u64, token_address: Address) -> bool {
        let registries = self.registries.read().await;
        if let Some(registry) = registries.get(&network_id) {
            registry.read().await.contains_token(token_address)
        } else {
            false
        }
    }

    pub async fn get_token(
        &self,
        network_id: u64,
        token_address: Address,
    ) -> Option<crate::models::token::Token> {
        let registries = self.registries.read().await;
        if let Some(registry) = registries.get(&network_id) {
            registry.read().await.get_token(token_address).cloned()
        } else {
            None
        }
    }

    pub async fn add_token(
        &self,
        network_id: u64,
        token: crate::models::token::Token,
    ) -> Result<(), String> {
        let registries = self.registries.read().await;
        if let Some(registry) = registries.get(&network_id) {
            let mut registry_guard = registry.write().await;
            registry_guard
                .add_token_with_network_id(token)
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!(
                "No token registry found for network {}",
                network_id
            ))
        }
    }
}

impl Clone for MultichainTokenRegistry {
    fn clone(&self) -> Self {
        Self {
            registries: Arc::clone(&self.registries),
        }
    }
}
