use std::collections::HashMap;

use async_trait::async_trait;

use crate::{sealed::Sealed, AgentService, Client, ConsulResult, QueryOptions};

/// A registered service health check. Returned with its associated
/// [ServiceEntry] instance by [Health::list_service_instances].
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct HealthCheck {
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub output: String,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    pub servicename: String,
    pub servicetags: Option<Vec<String>>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct Node {
    #[serde(rename = "ID")]
    pub id: String,
    pub node: String,
    pub address: String,
    pub datacenter: Option<String>,
    pub taggedaddresses: Option<HashMap<String, String>>,
    pub meta: Option<HashMap<String, String>>,
    pub createindex: u64,
    pub modifyindex: u64,
}

/// An [AgentService] with its associated [HealthCheck]s.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct ServiceEntry {
    /// The node the service is associated with.
    pub node: Node,
    /// The service configuration.
    pub service: AgentService,
    /// The health checks associated with the service.
    pub checks: Vec<HealthCheck>,
}

/// This trait provides methods for interacting with the `/health` endpoints.
#[async_trait]
pub trait Health: Sealed {
    /// This endpoint returns the checks associated with the service provided on
    /// the path.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/health#list-nodes-for-service
    async fn list_service_instances(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<ServiceEntry>>;
}

#[async_trait]
impl Health for Client {
    #[tracing::instrument]
    async fn list_service_instances(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<ServiceEntry>> {
        let mut params = HashMap::new();
        let path = format!("/v1/health/service/{}", service);
        if passing_only {
            params.insert(String::from("passing"), String::from("1"));
        }
        if let Some(tag) = tag {
            params.insert(String::from("tag"), tag.to_owned());
        }
        self.get(&path, options).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, Config, Health};

    #[tokio::test]
    async fn test_list_service_instances() {
        let config = Config::default();
        let client = Client::new(config);
        // An existing service for a agent in dev mode
        let snodes = client
            .list_service_instances("consul", Option::None, true, Option::None)
            .await
            .unwrap();
        {
            assert!(!snodes.is_empty(), "should have at least one Service Node");
        }
        // A non existing, should be empty
        let snodes = client
            .list_service_instances("non-existing-service", Option::None, true, Option::None)
            .await
            .unwrap();
        {
            assert_eq!(snodes.len(), 0);
        }
    }
}
