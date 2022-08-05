use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;

use crate::{sealed::Sealed, Client, ConsulResult, HealthCheck, ServiceWeights, TaggedAddress};

/// A service registered with the local agent.
///
/// This service was either provided through configuration files or added
/// dynamically using the HTTP API.
///
/// See the [List Services] endpoint documentation for more information.
///
/// [List Services]: https://www.consul.io/api-docs/agent/service#list-services
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    /// Identifies the service as a Connect proxy. See Connect
    /// for details.
    pub kind: String,
    /// Specifies the service ID. If this was not specified
    /// when the service was created, the value of the name field will be
    /// used.
    #[serde(rename = "ID")]
    pub id: String,
    /// List of string values that used to add service-level labels.
    pub tags: Option<Vec<String>>,
    /// Object that defines a map of the max 64 key/value pairs.
    /// The meta object has the same limitations as the node meta object in the
    /// node definition.
    pub meta: Option<HashMap<String, String>>,
    /// String value that specifies a service-specific IP address or hostname.
    pub address: String,
    /// Additional addresses defined for the service.
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    /// Specifies a service-specific port number.
    pub port: u16,
    /// Determines if the anti-entropy feature for the service is enabled.
    pub enable_tag_override: Option<bool>,
    /// Struct that configures the weight of the service in terms of its DNS
    /// service (SRV) response.
    pub weights: ServiceWeights,
}

/// Response returned by [AgentServices::get_local_service_config]. Identical to
/// [Service], but with the `content_hash` field.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceConfig {
    /// Identifies the service as a Connect proxy. See Connect
    /// for details.
    pub kind: String,
    /// Specifies the service ID. If this was not specified
    /// when the service was created, the value of the name field will be
    /// used.
    #[serde(rename = "ID")]
    pub id: String,
    /// Undocumented field that appears to mirror `ID`.
    pub service: String,
    /// List of string values that used to add service-level labels.
    pub tags: Option<Vec<String>>,
    /// Object that defines a map of the max 64 key/value pairs.
    /// The meta object has the same limitations as the node meta object in the
    /// node definition.
    pub meta: Option<HashMap<String, String>>,
    /// String value that specifies a service-specific IP address or hostname.
    pub address: String,
    /// Additional addresses defined for the service.
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    /// Specifies a service-specific port number.
    pub port: u16,
    /// Determines if the anti-entropy feature for the service is enabled.
    pub enable_tag_override: Option<bool>,
    /// Struct that configures the weight of the service in terms of its DNS
    /// service (SRV) response.
    pub weights: ServiceWeights,
    /// Contains the hash-based blocking query hash for the result.
    pub content_hash: String,
    // TODO: add proxy field
    // pub proxy: Proxy
}

/// Defines the configuration of a service to be created. Used by the
/// [AgentServices::register_service] method.
#[derive(Serialize, Default, Debug)]
pub struct ServiceRegistrationPayload {
    #[serde(rename = "Name")]
    /// Specifies the logical name of the service.
    pub name: String,
    /// Specifies a unique ID for this service. This must be unique per agent.
    /// This defaults to the Name parameter if not provided.
    #[serde(rename = "ID")]
    pub id: Option<String>,
    ///  Specifies a list of tags to assign to the service.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    /// Specifies the port on which the service is exposed.
    #[serde(rename = "Port")]
    pub port: u16,
    /// Specifies the address on which the service is exposed.
    #[serde(rename = "Address")]
    pub address: Option<String>,
    ///Specifies to disable the anti-entropy feature for this service's tags.
    #[serde(rename = "EnableTagOverride")]
    pub enable_tag_override: bool,
}
/// This trait provides methods for interacting with the `/agent/service`
/// endpoints.
///
/// This endpoint interact with services on the local agent in
/// Consul. These should not be confused with services in the catalog.
#[async_trait]
pub trait AgentServices: Sealed {
    /// This method returns all the services that are registered with the
    /// local agent. These services were either provided through configuration
    /// files or added dynamically using the HTTP API.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent/service#list-services
    async fn list_local_services(&self) -> ConsulResult<Vec<Service>>;

    /// This method returns the full service definition for a single service
    /// instance registered on the local agent.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API Documentation]: https://www.consul.io/api-docs/agent/service#get-service-configuration
    async fn get_local_service_config<S: AsRef<str> + Send + Debug>(
        &self,
        id: S,
    ) -> ConsulResult<ServiceConfig>;

    /// This method retrieves an aggregated state of service(s) on the local
    /// agent by name.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent/service#get-local-service-health
    async fn get_local_service_health<S: AsRef<str> + Send + Debug>(
        &self,
        name: S,
    ) -> ConsulResult<HealthCheck>;

    /// This method retrieves the health state of a specific service on the
    /// local agent by ID.
    async fn get_local_service_health_by_id<S: AsRef<str> + Send + Debug>(
        &self,
        id: S,
    ) -> ConsulResult<HealthCheck>;

    /// This endpoint adds a new service, with optional health checks, to the
    /// local agent.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent/service#register-service
    async fn register_service(&self, payload: ServiceRegistrationPayload) -> ConsulResult<()>;
}

#[async_trait]
impl AgentServices for Client {
    #[tracing::instrument]
    async fn list_local_services(&self) -> ConsulResult<Vec<Service>> {
        self.get("/v1/agent/services", None).await
    }

    #[tracing::instrument]
    async fn get_local_service_config<S: AsRef<str> + Send + Debug>(
        &self,
        name: S,
    ) -> ConsulResult<ServiceConfig> {
        self.get(format!("/v1/agent/services/{}", name.as_ref()), None).await
    }

    #[tracing::instrument]
    async fn get_local_service_health<S: AsRef<str> + Send + Debug>(
        &self,
        name: S,
    ) -> ConsulResult<HealthCheck> {
        self.get(format!("/v1/agent/health/service/{}", name.as_ref()), None).await
    }

    #[tracing::instrument]
    async fn get_local_service_health_by_id<S: AsRef<str> + Send + Debug>(
        &self,
        id: S,
    ) -> ConsulResult<HealthCheck> {
        self.get(format!("/v1/agent/health/service/id/{}", id.as_ref()), None).await
    }

    #[tracing::instrument]
    async fn register_service(&self, payload: ServiceRegistrationPayload) -> ConsulResult<()> {
        self.put("/v1/agent/service/register", payload, None, None).await
    }
}
