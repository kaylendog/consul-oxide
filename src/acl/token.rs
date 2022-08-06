use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

use super::{AclServiceIdentity, ConsulAcl, Policy};
use crate::ConsulResult;

/// Request payload for the [AclTokens::create_token] method.
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

/// Request payload for the [AclTokens::update_token] method.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UpdateToken {
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
    pub local: Option<bool>,
    /// If set this represents the point after which a token should be
    /// considered revoked and is eligible for destruction.
    pub expiration_time: Option<String>,
    /// This is a convenience field and if set will initialize the
    /// `expiration_time` field to a value of `create_time + expiration_ttl`.
    pub expiration_ttl: Option<String>,
}

#[async_trait]
trait AclTokens {
    /// This endpoint creates a new ACL token.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#create-a-token
    async fn create_token(&self, create_token: &CreateToken) -> ConsulResult<ConsulAcl>;

    /// This method reads an ACL token with the given Accessor ID.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#read-a-token
    async fn read_token(&self, token_id: &str) -> ConsulResult<ConsulAcl>;

    /// This method returns the ACL token details that matches the secret ID
    /// specified with the client's token or the token query parameter.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#read-self-token
    async fn read_self_token(&self) -> ConsulResult<ConsulAcl>;

    /// This method updates an existing ACL token.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#update-a-token
    async fn update_token(
        &self,
        accessor_id: &str,
        update_token: UpdateToken,
    ) -> ConsulResult<ConsulAcl>;

    /// This method clones an existing ACL token.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#clone-a-token
    async fn clone_token<S: AsRef<str>>(
        &self,
        accessor_id: S,
        new_description: Option<S>,
    ) -> ConsulResult<ConsulAcl>;

    /// This method deletes an ACL token.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#delete-a-token
    async fn delete_token<S: AsRef<str>>(&self, accessor_id: S) -> ConsulResult<bool>;

    /// This method lists all the ACL tokens.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/tokens#list-tokens
    async fn list_tokens(&self) -> ConsulResult<Vec<ConsulAcl>>;
}
