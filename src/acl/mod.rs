use std::collections::HashMap;

use async_trait::async_trait;
use serde::Deserialize;

use crate::{Client, ConsulResult};

mod token;

pub use token::*;

/// An access control list.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulAcl {
    #[serde(rename = "AccessorID")]
    pub accessor_id: String,
    #[serde(rename = "SecretID")]
    pub secret_id: String,
    pub description: String,
    /// A list of policies stored on this ACL.
    pub policies: Vec<Policy>,
    pub local: bool,
    pub create_time: String,
    pub hash: String,
    pub create_index: i64,
    pub modify_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
}

/// The ACL replication state of a datacenter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AclReplication {
    /// Reports whether ACL replication is enabled for the datacenter.
    pub enabled: bool,
    ///  Reports whether the ACL replication process is running. The process may
    /// take approximately 60 seconds to begin running after a leader election
    /// occurs.
    pub running: bool,
    pub source_datacenter: String,
    /// The type of replication that is currently in use.
    pub replication_type: ReplicationKind,
    /// The last index that was successfully replicated. Which data the
    /// replicated index refers to depends on the replication type.
    pub replicated_index: i64,
    /// The last token index that was successfully replicated.
    pub replicated_token_index: i64,
    /// The UTC time of the last successful sync operation. Since ACL
    /// replication is done with a blocking query, this may not update for up to
    /// 5 minutes if there have been no ACL changes to replicate. A zero value
    /// of "0001-01-01T00:00:00Z" will be present if no sync has been
    /// successful.
    pub last_success: String,
    /// The UTC time of the last error encountered during a sync operation. If
    /// this time is later than LastSuccess, you can assume the replication
    /// process is not in a good state. A zero value of "0001-01-01T00:00:00Z"
    /// will be present if no sync has resulted in an error.
    pub last_error: String,
    /// The last error message produced at the time of LastError. An empty
    /// string indicates that no sync has resulted in an error.
    pub last_error_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicationKind {
    /// ACL replication is only replicating policies as token replication is
    /// disabled.
    Policies,
    /// ACL replication is replicating both policies and tokens.
    Tokens,
}

impl<'de> Deserialize<'de> for ReplicationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "policies" => ReplicationKind::Policies,
            "tokens" => ReplicationKind::Tokens,
            _ => return Err(serde::de::Error::custom("unknown replication type")),
        })
    }
}

/// A service identity block.
///
/// Service identities are used during the authorization process to
/// automatically generate a policy for the service(s) specified.
///
/// For more information, see the [API documentation].
///
/// [API documentation]: https://www.consul.io/docs/internals/acl.html#service-identity
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AclServiceIdentity {
    /// Specifies the name of the service you want to associate with the policy.
    pub service_name: String,
    /// Specifies the names of datacenters in which the service identity
    /// applies. This field is optional.
    pub datacenters: Option<Vec<String>>,
}

/// Request payload for the [Acls::login] method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginPayload {
    /// The name of the auth method to use for login.
    pub auth_method: String,
    /// The bearer token to present to the auth method during login for
    /// authentication purposes.
    pub bearer_token: String,
    /// Specifies arbitrary KV metadata linked to the token. Can be useful to
    /// track origins.
    pub meta: Option<HashMap<String, String>>,
}

#[async_trait]
pub trait Acl {
    /// This method does a special one-time bootstrap of the ACL system,
    /// making the first management token if the acl.tokens.initial_management
    /// configuration entry is not specified in the Consul server configuration
    /// and if the cluster has not been bootstrapped previously.
    async fn bootstrap_acls(&self) -> ConsulResult<Vec<ConsulAcl>>;

    /// This method returns the status of the ACL replication processes in the
    /// datacenter. This is intended to be used by operators or by automation
    /// checking to discover the health of ACL replication.
    async fn check_acl_replication(&self) -> ConsulResult<AclReplication>;

    /// This method is used to exchange an auth method bearer token for a
    /// newly-created Consul ACL token.
    async fn login_to_auth_method(&self, payload: LoginPayload) -> ConsulResult<ConsulAcl>;

    /// This method is used to destroy a token created via the [Acl::login]
    /// method. The token deleted is specified with the X-Consul-Token header or
    /// the token query parameter.
    async fn logout_from_auth_method(&self) -> ConsulResult<()>;
}

#[async_trait]
impl Acl for Client {
    async fn bootstrap_acls(&self) -> ConsulResult<Vec<ConsulAcl>> {
        self.put("/v1/acl/bootstrap", (), None, None).await
    }

    async fn check_acl_replication(&self) -> ConsulResult<AclReplication> {
        self.get("/v1/acl/replication", None).await
    }

    async fn login_to_auth_method(&self, payload: LoginPayload) -> ConsulResult<ConsulAcl> {
        self.post("/v1/acl/login", payload, None, None).await
    }

    async fn logout_from_auth_method(&self) -> ConsulResult<()> {
        self.post_with_empty("/v1/acl/logout", (), None, None).await.map(|_: Option<()>| ())
    }
}

#[cfg(test)]
mod tests {
    use super::Acl;
    use crate::{Client, Config};

    #[tokio::test]
    async fn test_check_acl_replication() {
        let config = Config::default();
        let client = Client::new(config);
        // this should error on the test instance
        // TODO: devise non-erroring test instance
        client.check_acl_replication().await.unwrap_err();
    }
}
