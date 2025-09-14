use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use alloy::primitives::{TxHash, U256};

pub struct Metrics {
    pub blocks_processed: AtomicU64,
    pub pools_updated: AtomicU64,
    pub opportunities_found: AtomicU64,
    pub simulation_time: AtomicU64,
    pub last_block_time: AtomicU64,
    pub opportunities: HashMap<(TxHash, u64), OpportunityMetrics>,
}

#[derive(Debug, Clone)]
pub struct OpportunityMetrics {
    pub source_tx_hash: TxHash,
    pub source_log_index: u64,
    pub received_at: u64,
    pub proccessed_at: u64,
    pub simulated_at: u64,
    pub sent_at: u64,
    pub executed_at: u64,
    pub steps: Vec<U256>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            blocks_processed: AtomicU64::new(0),
            pools_updated: AtomicU64::new(0),
            opportunities_found: AtomicU64::new(0),
            simulation_time: AtomicU64::new(0),
            last_block_time: AtomicU64::new(0),
            opportunities: HashMap::new(),
        }
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_opportunity(&mut self, source_tx_hash: TxHash, log_index: u64, received_at: u64) {
        self.opportunities.insert(
            (source_tx_hash, log_index),
            OpportunityMetrics {
                source_tx_hash,
                source_log_index: log_index,
                received_at,
                proccessed_at: 0,
                simulated_at: 0,
                sent_at: 0,
                executed_at: 0,
                steps: Vec::new(),
            },
        );
    }

    pub fn set_received_at(&mut self, source_tx_hash: TxHash, log_index: u64, received_at: u64) {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.received_at = received_at;
        }
    }

    pub fn set_proccessed_at(
        &mut self,
        source_tx_hash: TxHash,
        log_index: u64,
        proccessed_at: u64,
    ) {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.proccessed_at = proccessed_at;
        }
    }

    pub fn set_simulated_at(&mut self, source_tx_hash: TxHash, log_index: u64, simulated_at: u64) {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.simulated_at = simulated_at;
        }
    }

    pub fn set_steps<I>(&mut self, source_tx_hash: TxHash, log_index: u64, steps: I)
    where
        I: IntoIterator<Item = U256>,
    {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.steps = steps.into_iter().collect();
        }
    }

    pub fn set_sent_at(&mut self, source_tx_hash: TxHash, log_index: u64, sent_at: u64) {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.sent_at = sent_at;
        }
    }

    pub fn set_executed_at(&mut self, source_tx_hash: TxHash, log_index: u64, executed_at: u64) {
        if let Some(opportunity) = self.opportunities.get_mut(&(source_tx_hash, log_index)) {
            opportunity.executed_at = executed_at;
        }
    }

    pub fn get_opportunity_metrics(
        &self,
        source_tx_hash: TxHash,
        log_index: u64,
    ) -> Option<&OpportunityMetrics> {
        self.opportunities.get(&(source_tx_hash, log_index))
    }

    pub fn get_opportunity_metrics_clone(
        &self,
        source_tx_hash: TxHash,
        log_index: u64,
    ) -> Option<OpportunityMetrics> {
        self.opportunities
            .get(&(source_tx_hash, log_index))
            .cloned()
    }

    pub fn drop_opportunity(&mut self, source_tx_hash: TxHash, log_index: u64) {
        self.opportunities.remove(&(source_tx_hash, log_index));
    }

    pub fn format_opportunity_metrics(&self, tx_hash: TxHash, log_index: u64) -> String {
        if let Some(opportunity) = self.opportunities.get(&(tx_hash, log_index)) {
            let proccess_duration = opportunity
                .proccessed_at
                .saturating_sub(opportunity.received_at);
            let simulate_duration = opportunity
                .simulated_at
                .saturating_sub(opportunity.proccessed_at);
            let send_duration = opportunity.sent_at.saturating_sub(opportunity.simulated_at);
            let execute_duration = opportunity.executed_at.saturating_sub(opportunity.sent_at);
            let total_duration = opportunity.sent_at.saturating_sub(opportunity.received_at);

            format!(
                    "TX: {} | Received at: {} | Proccessed at: {} (+{}ms) | Simulated at: {} (+{}ms) | Sent at: {} (+{}ms) | Executed at: {} (+{}ms) | Total time: {}ms\n | Steps: {}",
                    tx_hash,
                    chrono::DateTime::from_timestamp_millis(opportunity.received_at as i64).unwrap().format("%Y-%m-%d %H:%M:%S%.3f"),
                    chrono::DateTime::from_timestamp_millis(opportunity.proccessed_at as i64).unwrap().format("%Y-%m-%d %H:%M:%S%.3f"),
                    proccess_duration,
                    chrono::DateTime::from_timestamp_millis(opportunity.simulated_at as i64).unwrap().format("%Y-%m-%d %H:%M:%S%.3f"),
                    simulate_duration,
                    chrono::DateTime::from_timestamp_millis(opportunity.sent_at as i64).unwrap().format("%Y-%m-%d %H:%M:%S%.3f"),
                    send_duration,
                    chrono::DateTime::from_timestamp_millis(opportunity.executed_at as i64).unwrap().format("%Y-%m-%d %H:%M:%S%.3f"),
                    execute_duration,
                    total_duration,
                    opportunity.steps.iter().map(|step| step.to_string()).collect::<Vec<String>>().join(", ")
            )
        } else {
            "".to_string()
        }
    }

    pub fn increment_blocks_processed(&self) {
        self.blocks_processed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_pools_updated(&self) {
        self.pools_updated.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_opportunities_found(&self) {
        self.opportunities_found.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_simulation_time(&self, start_time: Instant) {
        let duration = start_time.elapsed().as_micros() as u64;
        self.simulation_time.fetch_add(duration, Ordering::Relaxed);
    }

    pub fn update_last_block_time(&self) {
        self.last_block_time.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            Ordering::Relaxed,
        );
    }

    pub fn get_metrics(&self) -> String {
        format!(
            "Blocks processed: {}\nPools updated: {}\nOpportunities found: {}\nAverage simulation time: {}ms\nLast block time: {}",
            self.blocks_processed.load(Ordering::Relaxed),
            self.pools_updated.load(Ordering::Relaxed),
            self.opportunities_found.load(Ordering::Relaxed),
            self.simulation_time.load(Ordering::Relaxed) / self.blocks_processed.load(Ordering::Relaxed).max(1),
            self.last_block_time.load(Ordering::Relaxed)
        )
    }
}
