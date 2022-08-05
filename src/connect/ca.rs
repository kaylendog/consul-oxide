use async_trait::async_trait;
use serde_json::Value;

use crate::{sealed::Sealed, Client, ConsulResult, QueryOptions};

/// Response payload for the [ConnectCA::get_ca_config] method.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CAConfig {
    #[serde(rename = "Provider")]
    provider: String,
    #[serde(rename = "Config")]
    config: Value,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

/// Response payload for the [ConnectCA::list_ca_root_certs] method.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARootList {
    #[serde(rename = "ActiveRootID")]
    active_root_id: String,
    #[serde(rename = "TrustDomain")]
    trust_domain: String,
    #[serde(rename = "Roots")]
    roots: Vec<CARoot>,
}

/// Entry in the root certificate list. Returned by the
/// [ConnectCA::list_ca_root_certs] method.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct CARoot {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "RootCert")]
    root_cert: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

/// This trait provides implementations of the Consul `/connect/ca` endpoint.
///
/// These endpoints provide tools for interacting with Connect's Certificate
/// Authority mechanism.
///
/// For more information, see the [API documentation](https://www.consul.io/api-docs/connect/ca).
#[async_trait]
pub trait ConnectCA: Sealed {
    /// This method returns the current list of trusted CA root certificates in
    /// the cluster.
    ///
    /// Fore more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#list-ca-root-certificates
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList>;

    /// This method returns the current CA configuration.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#get-ca-configuration
    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig>;

    /// This method updates the configuration for the CA.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/connect/ca.html#update-ca-configuration
    async fn update_ca_config(
        &self,
        conf: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()>;
}

#[async_trait]
impl ConnectCA for Client {
    #[tracing::instrument]
    async fn list_ca_root_certs(&self, options: Option<QueryOptions>) -> ConsulResult<CARootList> {
        self.get("/v1/connect/ca/roots", options).await
    }

    #[tracing::instrument]
    async fn get_ca_config(&self, options: Option<QueryOptions>) -> ConsulResult<CAConfig> {
        self.get("/v1/connect/ca/configuration", options).await
    }

    #[tracing::instrument]
    async fn update_ca_config(
        &self,
        payload: CAConfig,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/connect/ca/configuration", payload, None, options).await
    }
}
