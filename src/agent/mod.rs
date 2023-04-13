//! Defines methods for interacting with the Consul Agent API.

use std::sync::Arc;

use crate::http::Http;

mod services;

/// The `Agent` struct is used to interact with the agent endpoint of the Consul
/// HTTP API.
pub struct Agent {
    client: Arc<reqwest::Client>,
    config: Arc<crate::Config>,
    pub services: services::AgentServices,
}

impl Http for Agent {
    fn inner(&self) -> (&reqwest::Client, &crate::Config) {
        (&self.client, &self.config)
    }
}

impl Agent {
    pub(crate) fn new(client: Arc<reqwest::Client>, config: Arc<crate::Config>) -> Self {
        let services = services::AgentServices::new(client.clone(), config.clone());
        Self { client, config, services }
    }
}
