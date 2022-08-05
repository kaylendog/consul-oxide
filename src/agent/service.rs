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
pub struct Service {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "TaggedAddresses")]
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    #[serde(rename = "Meta")]
    pub meta: HashMap<String, String>,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Weights")]
    pub weights: ServiceWeights,
    #[serde(rename = "EnableTagOverride")]
    pub enable_tag_override: bool,
    #[serde(rename = "Address")]
    pub address: String,
}

/// The full service definition for a single service instance registered on the
/// local agent.
#[derive(Deserialize, Debug)]
pub struct ServiceConfig {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: Service,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Meta")]
    pub meta: Option<HashMap<String, String>>,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "TaggedAddresses")]
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    #[serde(rename = "Port")]
    pub port: u16,
}

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
