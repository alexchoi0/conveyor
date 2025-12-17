use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::service_registry::RegisteredService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadBalanceStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRandom,
    ConsistentHash,
}

impl Default for LoadBalanceStrategy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

pub struct LoadBalancer {
    round_robin_counters: Arc<RwLock<HashMap<String, AtomicUsize>>>,
    connection_counts: Arc<RwLock<HashMap<String, usize>>>,
    weights: Arc<RwLock<HashMap<String, u32>>>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            round_robin_counters: Arc::new(RwLock::new(HashMap::new())),
            connection_counts: Arc::new(RwLock::new(HashMap::new())),
            weights: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn select(
        &self,
        services: &[RegisteredService],
        strategy: LoadBalanceStrategy,
        routing_key: Option<&str>,
    ) -> Option<RegisteredService> {
        if services.is_empty() {
            return None;
        }

        if services.len() == 1 {
            return Some(services[0].clone());
        }

        match strategy {
            LoadBalanceStrategy::RoundRobin => self.round_robin(services).await,
            LoadBalanceStrategy::LeastConnections => self.least_connections(services).await,
            LoadBalanceStrategy::WeightedRandom => self.weighted_random(services).await,
            LoadBalanceStrategy::ConsistentHash => {
                self.consistent_hash(services, routing_key).await
            }
        }
    }

    async fn round_robin(&self, services: &[RegisteredService]) -> Option<RegisteredService> {
        if services.is_empty() {
            return None;
        }

        let key = services
            .first()
            .map(|s| s.service_name.clone())
            .unwrap_or_default();

        let mut counters = self.round_robin_counters.write().await;
        let counter = counters
            .entry(key)
            .or_insert_with(|| AtomicUsize::new(0));

        let idx = counter.fetch_add(1, Ordering::Relaxed) % services.len();
        Some(services[idx].clone())
    }

    async fn least_connections(
        &self,
        services: &[RegisteredService],
    ) -> Option<RegisteredService> {
        if services.is_empty() {
            return None;
        }

        let counts = self.connection_counts.read().await;

        let mut min_connections = usize::MAX;
        let mut selected = &services[0];

        for service in services {
            let count = counts.get(&service.service_id).copied().unwrap_or(0);
            if count < min_connections {
                min_connections = count;
                selected = service;
            }
        }

        Some(selected.clone())
    }

    async fn weighted_random(&self, services: &[RegisteredService]) -> Option<RegisteredService> {
        if services.is_empty() {
            return None;
        }

        let weights = self.weights.read().await;

        let mut total_weight: u32 = 0;
        let mut service_weights = Vec::with_capacity(services.len());

        for service in services {
            let weight = weights.get(&service.service_id).copied().unwrap_or(100);
            total_weight += weight;
            service_weights.push((service, weight));
        }

        if total_weight == 0 {
            return Some(services[0].clone());
        }

        let random_point = rand_u32() % total_weight;
        let mut cumulative = 0u32;

        for (service, weight) in service_weights {
            cumulative += weight;
            if random_point < cumulative {
                return Some(service.clone());
            }
        }

        Some(services[0].clone())
    }

    async fn consistent_hash(
        &self,
        services: &[RegisteredService],
        routing_key: Option<&str>,
    ) -> Option<RegisteredService> {
        if services.is_empty() {
            return None;
        }

        let key = routing_key.unwrap_or("default");
        let hash = simple_hash(key);

        let idx = (hash as usize) % services.len();
        Some(services[idx].clone())
    }

    pub async fn increment_connections(&self, service_id: &str) {
        let mut counts = self.connection_counts.write().await;
        *counts.entry(service_id.to_string()).or_insert(0) += 1;
    }

    pub async fn decrement_connections(&self, service_id: &str) {
        let mut counts = self.connection_counts.write().await;
        if let Some(count) = counts.get_mut(service_id) {
            *count = count.saturating_sub(1);
        }
    }

    pub async fn set_weight(&self, service_id: &str, weight: u32) {
        let mut weights = self.weights.write().await;
        weights.insert(service_id.to_string(), weight);
    }
}

fn simple_hash(s: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn rand_u32() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}
