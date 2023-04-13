use std::sync::Arc;

use crate::http::Http;

pub struct AgentServices {
    client: Arc<reqwest::Client>,
    config: Arc<crate::Config>,
}

impl Http for AgentServices {
    fn inner(&self) -> (&reqwest::Client, &crate::Config) {
        (&self.client, &self.config)
    }
}

impl AgentServices {
    pub(crate) fn new(client: Arc<reqwest::Client>, config: Arc<crate::Config>) -> Self {
        Self { client, config }
    }
}
