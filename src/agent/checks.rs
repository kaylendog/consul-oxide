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
#[derive(Serialize, Default)]
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
    pub deregister_critical_service_after: Option<String>,
    /// Specifies the ID of the node for an alias check. If no service is
    /// specified, the check will alias the health of the node. If a service is
    /// specified, the check will alias the specified service on this particular
    /// node.
    pub alias_node: Option<String>,
    ///  Specifies the ID of a service for an alias check. If the service is not
    /// registered with the same agent, AliasNode must also be specified. Note
    /// this is the service ID and not the service name (though they are very
    /// often the same).
    pub alias_service: Option<String>,
    /// Specifies the ID of a service to associate the registered check with an
    /// existing service provided by the agent.
    pub service_id: Option<String>,
    /// Specifies the initial status of the health check.
    pub status: Option<String>,

    /// pecifies the number of consecutive successful results required before
    /// check status transitions to passing. Available for HTTP, TCP, gRPC,
    /// Docker & Monitor checks. Added in Consul 1.7.0.
    pub success_before_passing: Option<u8>,
    /// Specifies the number of consecutive unsuccessful results required before
    /// check status transitions to warning. Defaults to the same value as
    /// FailuresBeforeCritical. Values higher than FailuresBeforeCritical are
    /// invalid. Available for HTTP, TCP, gRPC, Docker & Monitor checks. Added
    /// in Consul 1.11.0.
    pub failures_before_warning: Option<u8>,
    /// Specifies the number of consecutive unsuccessful results required before
    /// check status transitions to critical. Available for HTTP, TCP, gRPC,
    /// Docker & Monitor checks. Added in Consul 1.7.0.
    pub failures_before_critical: Option<u8>,

    /// Specifies command arguments to run to update the status of the check.
    /// Prior to Consul 1.0, checks used a single Script field to define the
    /// command to run, and would always run in a shell. In Consul 1.0, the Args
    /// array was added so that checks can be run without a shell. The Script
    /// field is deprecated, and you should include the shell in the Args to run
    /// under a shell, eg. "args": ["sh", "-c", "..."].
    pub args: Vec<String>,
    /// Specifies that the check is a Docker check, and Consul will evaluate the
    /// script every Interval in the given container using the specified Shell.
    /// Note that Shell is currently only supported for Docker checks.
    pub docker_container_id: Option<String>,
    /// Used alongside `docker_container_id` to specify the shell to use when
    /// evaluating the script inside the given container.
    pub shell: Option<String>,

    /// Specifies an HTTP check to perform a GET request against the value of
    /// HTTP (expected to be a URL) every Interval. If the response is any 2xx
    /// code, the check is passing. If the response is 429 Too Many Requests,
    /// the check is warning. Otherwise, the check is critical. HTTP checks also
    /// support SSL. By default, a valid SSL certificate is expected.
    /// Certificate verification can be controlled using the TLSSkipVerify.
    pub http: Option<String>,
    /// Specifies a different HTTP method to be used for an HTTP check. When no
    /// value is specified, GET is used.
    pub method: Option<String>,
    /// Specifies a set of headers that should be set for HTTP checks. Each
    /// header can have multiple values.
    pub header: Option<HashMap<String, Vec<String>>>,
    /// Specifies a body that should be sent with `http` checks.
    pub body: Option<String>,
    /// Specifies whether to disable following HTTP redirects when performing an
    /// `HTTP` check.
    pub disable_redirects: bool,
    /// Specifies the frequency at which to run this check. This is required for
    /// HTTP and TCP checks.
    pub interval: Option<String>,
    /// Specifies a timeout for outgoing connections in the case of a Script,
    /// HTTP, TCP, or gRPC check. Can be specified in the form of "10s" or "5m"
    /// (i.e., 10 seconds or 5 minutes, respectively).
    pub timeout: String,

    /// Specifies if the certificate for an HTTPS check should not be verified.
    #[serde(rename = "TLSSkipVerify")]
    pub tlsskip_verify: bool,

    /// Specifies a `gRPC` check's endpoint that supports the standard gRPC
    /// health checking protocol. The state of the check will be updated at
    /// the given `interval` by probing the configured endpoint. Add the
    /// service identifier after the `gRPC` check's endpoint in the
    /// following format to check for a specific service instead of the
    /// whole gRPC server `/:service_identifier`.
    #[serde(rename = "GRPC")]
    pub grpc: Option<String>,
    /// Specifies whether to use TLS for this `gRPC` health check. If TLS is
    /// enabled, then by default, a valid TLS certificate is expected.
    /// Certificate verification can be turned off by setting `tls_skip_verify`
    /// to `true`.
    #[serde(rename = "GRPCUseTLS")]
    pub gprc_use_tls: Option<bool>,

    /// Specifies an address that uses http2 to run a ping check on. At the
    /// specified Interval, a connection is made to the address, and a ping is
    /// sent. If the ping is successful, the check will be classified as
    /// `passing`, otherwise it will be marked as `critical`. TLS is used by
    /// default. To disable TLS and use h2c, set `h2_ping_use_tls` to `false`.
    /// If TLS is enabled, a valid SSL certificate is required by default,
    /// but verification can be removed with `tls_skip_verify`.
    #[serde(rename = "H2Ping")]
    pub h2_ping: Option<String>,
    /// Specifies if TLS should be used for H2PING check. If TLS is enabled, a
    /// valid SSL certificate is required by default, but verification can be
    /// removed with `tls_skip_verify`.
    #[serde(rename = "H2PingUseTLS")]
    pub h2_ping_use_tls: Option<bool>,

    ///  Specifies a TCP to connect against the value of TCP (expected to be an
    /// IP or hostname plus port combination) every Interval. If the connection
    /// attempt is successful, the check is passing. If the connection attempt
    /// is unsuccessful, the check is critical. In the case of a hostname that
    /// resolves to both IPv4 and IPv6 addresses, an attempt will be made to
    /// both addresses, and the first successful connection attempt will result
    /// in a successful check.
    #[serde(rename = "TCP")]
    pub tcp: Option<String>,

    /// Specifies this is a TTL check, and the TTL endpoint must be used
    /// periodically to update the state of the check. If the check is not set
    /// to passing within the specified duration, then the check will be set to
    /// the failed state.
    pub ttl: Option<String>,
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

    /// This method deregisters a check with the local agent.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/agent/check.html#deregister-check
    async fn deregister_check(&self, check_id: &str) -> ConsulResult<()>;
}

#[async_trait]
impl AgentChecks for Client {
    async fn list_checks(&self) -> ConsulResult<HashMap<String, AgentCheck>> {
        self.get("/v1/agent/checks", None).await
    }
    async fn register_check(&self, check: RegisterCheckPayload) -> ConsulResult<()> {
        self.put("/v1/agent/check/register", check, None, None).await
    }
    async fn deregister_check(&self, check_id: &str) -> ConsulResult<()> {
        self.put(&format!("/v1/agent/check/deregister/{}", check_id), (), None, None).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{AgentChecks, Client, Config};

    #[tokio::test]
    async fn test_list_checks() {
        let client = Client::new(Config::default());
        let result = client.list_checks().await.unwrap();
        assert_eq!(result.len(), 0);
        println!("{:?}", result);
    }
}
