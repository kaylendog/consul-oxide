use serde_derive::{Deserialize, Serialize};

use super::{AclServiceIdentity, Policy};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateToken {
    /// Specifies a UUID to use as the token's Accessor ID. If not specified a
    /// UUID will be generated for this field.
    pub accessor_id: Option<String>,
    /// Specifies a UUID to use as the token's Secret ID. If not specified a
    /// UUID will be generated for this field. Added in v1.5.0.
    pub secret_id: Option<String>,
    /// Free form human readable description of the token.
    pub description: Option<String>,
    /// The list of policies that should be applied to the token.
    pub policies: Option<Vec<Policy>>,
    /// The list of roles that should be applied to the token.
    pub roles: Option<Vec<RoleLink>>,
    /// The list of service identities that should be applied to the token.
    pub service_identities: Option<Vec<AclServiceIdentity>>,
    /// If true, indicates that the token should not be replicated globally and
    /// instead be local to the current datacenter.
    pub local: bool,
    /// If set this represents the point after which a token should be
    /// considered revoked and is eligible for destruction.
    pub expiration_time: Option<String>,
    /// This is a convenience field and if set will initialize the
    /// `expiration_time` field to a value of `create_time + expiration_ttl`.
    pub expiration_ttl: Option<String>,
}

/// A node identity configuration block. Returned
///
/// Node identities are configuration blocks that you can add to role
/// configurations or specify when linking tokens to policies.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeIdentity {
    /// The name of the node.
    pub node_name: String,
    ///  Specifies the node's datacenter.
    pub datacenter: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoleLink {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
}
