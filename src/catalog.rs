use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    agent::AgentService, sealed::Sealed, AgentCheck, Client, ConsulResult, QueryOptions,
    ServiceWeights,
};

/// A node within the cluster gossip pool.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct Node {
    /// The ID of the node.
    #[serde(rename = "ID")]
    id: String,
    /// The name of the node.
    node: String,
    /// The address of the node.
    address: String,
    /// The datacenter of the node.
    datacenter: String,
    /// The tags of the node.
    tagged_addresses: HashMap<String, String>,
    /// The meta data of the node.
    meta: HashMap<String, String>,
    create_index: u64,
    modify_index: u64,
}

/// A service defined within the Agent catalog.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct CatalogService {
    /// The ID of the service.
    #[serde(rename = "ID")]
    id: String,
    /// The node the service is associated with.
    node: String,
    /// The address of the node.
    address: String,
    /// The datacenter of the node running the service.
    datacenter: String,
    /// A map of addresses tagged to the node hosting the service.
    tagged_addresses: HashMap<String, String>,
    /// Metadata attached to the node this service is hosted on.
    node_meta: HashMap<String, String>,
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    service_id: String,
    /// The name of the service.
    service_name: String,
    /// The address of the service.
    service_address: String,
    /// Tags assigned to the service.
    service_tags: Vec<String>,
    /// Metadata assigned to the service.
    service_meta: HashMap<String, String>,
    /// The port of the service.
    service_port: u32,
    service_weights: ServiceWeights,
    service_enable_tag_override: bool,
    create_index: u64,
    modify_index: u64,
}

/// A response datatype containing a [Node] and its services.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct CatalogNode {
    /// The node stored in the catalog.
    node: Option<Node>,
    /// The services associated with the node.
    services: HashMap<String, AgentService>,
}

/// Datatype containing payload data for the [crate::Catalog::register] method.
///
/// For more information, see the [API documentation](https://www.consul.io/api-docs/catalog#json-request-body-schema).
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct CatalogRegistrationPayload {
    /// An optional UUID to assign to the node. This must be a 36-character
    /// UUID-formatted string.
    #[serde(rename = "Node")]
    id: String,
    /// Specifies the node ID to register.
    node: String,
    /// Specifies the address to register.
    address: String,
    /// Specifies the tagged addresses.
    tagged_addresses: HashMap<String, String>,
    /// Specifies arbitrary KV metadata pairs for filtering purposes.
    node_meta: HashMap<String, String>,
    /// Specifies the datacenter, which defaults to the agent's datacenter if
    /// not provided.
    datacenter: String,
    /// Specifies to register a service. If `id` is not provided, it will be
    /// defaulted to the value of the Service.Service property. Only one service
    /// with a given ID may be present per node.
    service: Option<AgentService>,
    /// Specifies to register a check.
    check: Option<AgentCheck>,
    /// Specifies whether to skip updating the node's information in the
    /// registration.
    skip_node_update: bool,
}

/// Request payload datatype for the [crate::Catalog::deregister] method.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct CatalogDeregistrationPayload {
    /// Specifies the node ID to deregister.
    node: String,
    /// The address of the node.
    address: String,
    /// Specifies the datacenter, which defaults to the agent's datacenter if
    /// not provided.
    datacenter: String,
    /// Specifies the service ID to deregister.
    #[serde(rename = "ServiceID")]
    service_id: String,
    /// Specifies the check ID to deregister.
    #[serde(rename = "CheckID")]
    check_id: String,
}

/// This trait provides methods for interacting with the Agent catalogue.
#[async_trait]
pub trait Catalog: Sealed {
    /// This method is a low-level mechanism for registering or updating
    /// entries in the catalog. It is usually preferable to instead use methods
    /// defined in the `Agent` trait for registration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, see the [API documentation](https://www.consul.io/api-docs/catalog#register-entity).
    async fn register_entity(
        &self,
        reg: CatalogRegistrationPayload,
        q: Option<QueryOptions>,
    ) -> ConsulResult<()>;

    /// This method is a low-level mechanism for directly removing entries from
    /// the Catalog. It is usually preferable to instead use methods defined
    /// in the `Agent` trait for deregistration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, see the [API documentation](https://www.consul.io/api/catalog.html#deregister-entity).
    async fn deregister_entity(
        &self,
        payload: CatalogDeregistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()>;

    /// This method returns the list of all known datacenters. The datacenters
    /// will be sorted in ascending order based on the estimated median round
    /// trip time from the server to the servers in that datacenter.
    ///
    /// For more information, see the [API documentation](https://www.consul.io/api/catalog.html#list-datacenters).
    async fn list_datacenters(&self) -> ConsulResult<Vec<String>>;

    /// This endpoint and returns the nodes registered in a given datacenter.
    ///
    /// For more information, see the [API documentation](https://www.consul.io/api/catalog.html#list-nodes).
    async fn list_nodes(&self, q: Option<QueryOptions>)
        -> ConsulResult<HashMap<String, Vec<Node>>>;

    /// This endpoint returns the services registered in a given datacenter.
    ///
    /// For more information, see the [API documentation](https://www.consul.io/api-docs/catalog#list-services).
    async fn list_services(
        &self,
        q: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<String>>>;
}

#[async_trait]
impl Catalog for Client {
    #[tracing::instrument]
    async fn register_entity(
        &self,
        payload: CatalogRegistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/session/create", payload, None, options).await
    }

    #[tracing::instrument]
    async fn deregister_entity(
        &self,
        payload: CatalogDeregistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/catalog/deregister", payload, None, options).await
    }

    #[tracing::instrument]
    async fn list_datacenters(&self) -> ConsulResult<Vec<String>> {
        self.get("/v1/catalog/datacenters", None).await
    }

    #[tracing::instrument]
    async fn list_nodes(
        &self,
        q: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<Node>>> {
        self.get("/v1/catalog/nodes", q).await
    }

    #[tracing::instrument]
    async fn list_services(
        &self,
        options: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<String>>> {
        self.get("/v1/catalog/services", options).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Catalog, Client, Config};

    #[tokio::test]
    async fn test_list_datacenters() {
        let config = Config::new_from_env();
        let client = Client::new(config);
        let r = client.list_datacenters().await.unwrap();
        assert_eq!(r, ["dc1"]);
    }

    #[tokio::test]
    async fn test_list_services() {
        let config = Config::default();
        let client = Client::new(config);
        let r = client.list_services(None).await.unwrap();
        assert_ne!(r.len(), 0);
        match r.get("consul") {
            None => panic!("Should have a Consul service"),
            Some(val) => assert_eq!(val.len(), 0), // consul has no tags
        }
    }
}
