//! Defines methods for interacting with the catalog endpoint of the Consul HTTP
//! API.

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    common::{Node, TaggedAddress},
    http::Http,
    Result,
};

/// The `Catalog` struct is used to interact with the catalog endpoint of the
/// Consul HTTP API.
pub struct Catalog {
    client: Arc<reqwest::Client>,
    config: Arc<crate::Config>,
}

impl Http for Catalog {
    fn inner(&self) -> (&reqwest::Client, &crate::Config) {
        (&self.client, &self.config)
    }
}

impl Catalog {
    pub(crate) fn new(client: Arc<reqwest::Client>, config: Arc<crate::Config>) -> Self {
        Self { client, config }
    }

    /// This endpoint returns the list of all known datacenters. The datacenters
    /// will be sorted in ascending order based on the estimated median round
    /// trip time from the server to the servers in that datacenter.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-datacenters>
    pub async fn list_datacenters(&self) -> Result<Vec<String>> {
        self.get("/catalog/datacenters").await
    }

    /// This endpoint and returns the nodes registered in a given datacenter.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-nodes>
    pub async fn list_nodes(&self) -> Result<Vec<crate::common::Node>> {
        self.get("/catalog/nodes").await
    }

    /// This endpoint returns the services registered in a given datacenter.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-services>
    pub async fn list_services(&self) -> Result<HashMap<String, Vec<String>>> {
        self.get("/catalog/services").await
    }

    /// This endpoint returns the nodes providing a service in a given
    /// datacenter.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-nodes-for-service>
    pub async fn list_nodes_for_service(&self, service_name: &str) -> Result<Vec<ServiceNode>> {
        self.get(&format!("/catalog/service/{}", service_name)).await
    }

    /// This endpoint returns the nodes providing a Connect-capable service in a
    /// given datacenter. This will include both proxies and native
    /// integrations. A service may register both Connect-capable and incapable
    /// services at the same time, so this endpoint may be used to filter only
    /// the Connect-capable endpoints.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-nodes-for-connect-capable-service>
    pub async fn list_nodes_for_connect_capable_service(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceNode>> {
        self.get(&format!("/catalog/connect/{}", service_name)).await
    }

    /// This endpoint returns the node's registered services.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#retrieve-map-of-services-for-a-node>
    pub async fn get_node_service_map(&self, node_name: &str) -> Result<NodeServices> {
        self.get(&format!("/catalog/node/{}", node_name)).await
    }

    /// This endpoint returns the node's registered services.
    ///
    /// <https://developer.hashicorp.com/consul/api-docs/catalog#list-services-for-node>
    pub async fn list_node(&self, node_name: &str) -> Result<Vec<NodeService>> {
        self.get(&format!("/catalog/node-services/:node_name/{}", node_name)).await
    }
}

/// A node providing a service.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceNode {
    #[serde(rename = "ID")]
    pub id: String,
    pub node: String,
    /// The IP address of the Consul node on which the service is registered.
    pub address: String,
    /// The data center of the Consul node on which the service is registered.
    pub datacenter: String,
    /// The list of explicit LAN and WAN IP addresses for the agent
    pub tagged_addresses: HashMap<String, String>,
    /// A list of user-defined metadata key/value pairs for the node
    pub node_meta: HashMap<String, String>,
    /// An internal index value representing when the service was created
    pub create_index: i64,
    /// The last index that modified the service
    pub modify_index: i64,
    /// The IP address of the service host — if empty, node address should be
    /// used
    pub service_address: String,
    /// Indicates whether service tags can be overridden on this service
    pub service_enable_tag_override: bool,
    /// A unique service instance identifier
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    /// The name of the service
    pub service_name: String,
    /// The port number of the service
    pub service_port: i64,
    /// A list of user-defined metadata key/value pairs for the service
    pub service_meta: HashMap<String, String>,
    /// The map of explicit LAN and WAN addresses for the service instance. This
    /// includes both the address as well as the port.
    pub service_tagged_addresses: HashMap<String, TaggedAddress>,
    /// A list of tags for the service
    pub service_tags: Vec<String>,
    /// The proxy config as specified in Connect Proxies.
    pub service_proxy: ServiceProxy,
    /// The Connect settings. The value of this struct is equivalent to the
    /// Connect field for service registration.
    pub service_connect: ServiceConnect,
}

/// The proxy config as specified in Connect Proxies.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceProxy {
    pub destination_service_name: String,
    #[serde(rename = "DestinationServiceID")]
    pub destination_service_id: String,
    pub local_service_address: String,
    pub local_service_port: i64,
    pub config: Value,
    pub upstreams: Value,
}

/// The Connect settings. The value of this struct is equivalent to the Connect
/// field for service registration.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceConnect {
    pub native: bool,
    pub proxy: Value,
}

/// The node's registered services.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeServices {
    pub node: Node,
    pub services: HashMap<String, NodeService>,
}

/// A service registered with a node.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeService {
    #[serde(rename = "ID")]
    pub id: String,
    pub service: String,
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub port: u16,
}
