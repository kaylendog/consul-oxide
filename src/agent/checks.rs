use std::collections::HashMap;

use async_trait::async_trait;

use crate::{Client, ConsulResult};

/// A health check run on a service hosted on this node.
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentCheck {
    /// The node the service is running on.
    #[serde(rename = "Node")]
    pub node: String,
    /// The ID of the service within the agent.
    #[serde(rename = "CheckID")]
    pub check_id: String,
    /// The name of the service.
    #[serde(rename = "Name")]
    pub name: String,
    /// The status of the check.
    #[serde(rename = "Status")]
    pub status: String,
    /// Notes attached to this check.
    #[serde(rename = "Notes")]
    pub notes: String,
    /// Output of the check.
    #[serde(rename = "Output")]
    pub output: String,
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    /// The name of the service.
    #[serde(rename = "ServiceName")]
    pub service_name: String,
}

/// The request payload for the [`AgentChecks::register_check`] endpoint.
///
/// See the [API Documentation] for more information.
///
/// [API Documentation]: https://www.consul.io/api-docs/agent/check#json-request-body-schema
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterCheckPayload {
    /// Specifies a unique ID for this check on the node. This defaults to the
    /// "Name" parameter, but it may be necessary to provide an ID for
    /// uniqueness. This value will return in the response as "CheckId".
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// Specifies the name of the check.
    pub name: String,
    /// Specifies arbitrary information for humans. This is not used by Consul
    /// internally.
    pub notes: Option<String>,
    /// Specifies that checks associated with a service should deregister after
    /// this time. This is specified as a time duration with suffix like "10m".
    /// If a check is in the critical state for more than this configured value,
    /// then its associated service (and all of its associated checks) will
    /// automatically be deregistered. The minimum timeout is 1 minute, and the
    /// process that reaps critical services runs every 30 seconds, so it may
    /// take slightly longer than the configured timeout to trigger the
    /// deregistration. This should generally be configured with a timeout
    /// that's much, much longer than any expected recoverable outage for the
    /// given service.
    pub deregister_critical_service_after: String,
    /// Specifies command arguments to run to update the status of the check.
    /// Prior to Consul 1.0, checks used a single Script field to define the
    /// command to run, and would always run in a shell. In Consul 1.0, the Args
    /// array was added so that checks can be run without a shell. The Script
    /// field is deprecated, and you should include the shell in the Args to run
    /// under a shell, eg. "args": ["sh", "-c", "..."].
    pub args: Vec<String>,
    pub docker_container_id: String,
    pub shell: String,
    pub http: String,
    pub method: String,
    /// Specifies a set of headers that should be set for HTTP checks. Each
    /// header can have multiple values.
    pub header: Option<HashMap<String, Vec<String>>>,
    pub body: Option<String>,
    pub disable_redirects: bool,
    #[serde(rename = "TCP")]
    pub tcp: String,
    /// Specifies the frequency at which to run this check. This is required for
    /// HTTP and TCP checks.
    pub interval: Option<String>,
    /// Specifies a timeout for outgoing connections in the case of a Script,
    /// HTTP, TCP, or gRPC check. Can be specified in the form of "10s" or "5m"
    /// (i.e., 10 seconds or 5 minutes, respectively).
    pub timeout: String,
    #[serde(rename = "TLSSkipVerify")]
    pub tlsskip_verify: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCheckPayloadHeader {
    #[serde(rename = "Content-Type")]
    pub content_type: Vec<String>,
}

#[async_trait]
pub trait AgentChecks {
    /// This method returns all checks that are registered with the local
    /// agent.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/agent/check.html#list-checks
    async fn list_checks(&self) -> ConsulResult<HashMap<String, AgentCheck>>;

    /// This method registers a check with the local agent.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/agent/check.html#register-check
    async fn register_check(&self, check: RegisterCheckPayload) -> ConsulResult<()>;
}

#[async_trait]
impl AgentChecks for Client {
    async fn list_checks(&self) -> ConsulResult<HashMap<String, AgentCheck>> {
        self.get("/v1/agent/checks", None).await
    }
    async fn register_check(&self, check: RegisterCheckPayload) -> ConsulResult<()> {
        self.put("/v1/agent/check/register", check, None, None).await
    }
}
