//! Defines methods for interacting with the Consul Health API.

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    common::{Node, TaggedAddress},
    http::Http,
    Result,
};
/// This struct is used to interact with the health endpoint of the Consul HTTP
/// API.
pub struct Health {
    client: Arc<reqwest::Client>,
    config: Arc<crate::Config>,
}

impl Http for Health {
    fn inner(&self) -> (&reqwest::Client, &crate::Config) {
        (&self.client, &self.config)
    }
}

impl Health {
    pub fn new(client: Arc<reqwest::Client>, config: Arc<crate::Config>) -> Self {
        Self { client, config }
    }

    /// This endpoint returns the checks specific to the node provided on the
    /// path.
    //
    // <https://developer.hashicorp.com/consul/api-docs/health#list-checks-for-node>
    pub async fn list_checks_for_node(&self, node_name: &str) -> Result<Vec<Check>> {
        self.get(&format!("/health/node/{}", node_name)).await
    }

    /// This endpoint returns the checks associated with the service provided on
    /// the path.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/health#list-checks-for-service>
    pub async fn list_checks_for_service(&self, service_name: &str) -> Result<Vec<Check>> {
        self.get(&format!("/health/checks/{}", service_name)).await
    }

    /// This endpoint returns the service instances providing the service
    /// indicated on the path. Users can also build in support for dynamic load
    /// balancing and other features by incorporating the use of health checks.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/health#list-nodes-for-service>
    pub async fn list_service_instances(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        self.get(&format!("/health/service/{}", service_name)).await
    }

    /// This endpoint returns the checks in the state provided on the path.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/health#list-checks-in-state>
    pub async fn list_checks_in_state(&self, state: State) -> Result<Vec<Check>> {
        self.get(&format!("/health/state/{}", state.to_string())).await
    }
}

/// A health check associated with a service.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Check {
    #[serde(rename = "ID")]
    pub id: String,
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub output: String,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    pub service_name: String,
    pub service_tags: Vec<String>,
}

/// A service instance.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceInstance {
    /// The node on which the service instance is running.
    pub node: Node,
    /// The service definition.
    pub service: Service,
    /// The health checks associated with the service instance.
    pub checks: Vec<Check>,
}

/// A service definition.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    #[serde(rename = "ID")]
    pub id: String,
    pub service: String,
    pub tags: Vec<String>,
    pub address: String,
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    pub meta: HashMap<String, String>,
    pub port: u16,
    pub weights: Weights,
}

/// Weights are used to influence how often a service instance is selected for
/// a given service query.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weights {
    pub passing: i64,
    pub warning: i64,
}

/// The state of a health check.
pub enum State {
    Any,
    Passing,
    Warning,
    Critical,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Any => "any".to_string(),
            State::Passing => "passing".to_string(),
            State::Warning => "warning".to_string(),
            State::Critical => "critical".to_string(),
        }
    }
}
