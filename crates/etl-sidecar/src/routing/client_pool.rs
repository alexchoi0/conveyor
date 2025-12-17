use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, Context};
use tonic::transport::Channel;
use tracing::debug;

pub struct ClientPool<T: Clone> {
    clients: Arc<RwLock<HashMap<String, T>>>,
}

impl<T: Clone> ClientPool<T> {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create<F>(&self, endpoint: &str, create: F) -> Result<T>
    where
        F: FnOnce(Channel) -> T,
    {
        {
            let clients = self.clients.read().await;
            if let Some(client) = clients.get(endpoint) {
                return Ok(client.clone());
            }
        }

        debug!("Creating new connection to {}", endpoint);

        let channel = Channel::from_shared(format!("http://{}", endpoint))
            .context("Invalid endpoint")?
            .connect()
            .await
            .context("Failed to connect")?;

        let client = create(channel);

        {
            let mut clients = self.clients.write().await;
            clients.insert(endpoint.to_string(), client.clone());
        }

        Ok(client)
    }

    pub async fn remove(&self, endpoint: &str) {
        let mut clients = self.clients.write().await;
        if clients.remove(endpoint).is_some() {
            debug!("Removed client for {}", endpoint);
        }
    }

    pub async fn clear(&self) {
        let mut clients = self.clients.write().await;
        clients.clear();
    }
}

impl<T: Clone> Default for ClientPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
