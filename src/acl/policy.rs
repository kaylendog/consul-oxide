use std::fmt::Debug;

use async_trait::async_trait;

use crate::{Client, ConsulResult};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AclPolicy {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub description: String,
    pub rules: String,
    pub datacenters: Vec<String>,
    pub hash: String,
    pub create_index: i64,
    pub modify_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePolicy {
    /// Specifies a name for the ACL policy. The name can contain alphanumeric
    /// characters, dashes `-`, and underscores `_`. This name must be unique.
    pub name: String,
    /// Free form human readable description of the policy.
    pub description: String,
    /// Specifies rules for the ACL policy. The format of the Rules property is detailed in the [ACL Rules documentation](https://www.consul.io/docs/security/acl/acl-rules).
    pub rules: String,
    /// Specifies the datacenters the policy is valid within. When no
    /// datacenters are provided the policy is valid in all datacenters
    /// including those which do not yet exist but may in the future.
    pub datacenters: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdatePolicy {
    /// Specifies a name for the ACL policy. The name can contain alphanumeric
    /// characters, dashes `-`, and underscores `_`. This name must be unique.
    pub name: String,
    /// Free form human readable description of the policy.
    pub description: Option<String>,
    /// Specifies rules for the ACL policy. The format of the Rules property is detailed in the [ACL Rules documentation](https://www.consul.io/docs/security/acl/acl-rules).
    pub rules: Option<String>,
    /// Specifies the datacenters the policy is valid within. When no
    /// datacenters are provided the policy is valid in all datacenters
    /// including those which do not yet exist but may in the future.
    pub datacenters: Option<Vec<String>>,
}

#[async_trait]
pub trait AclPolicies {
    /// This method creates a new ACL policy.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#create-a-policy
    async fn create_policy(&self, payload: CreatePolicy) -> ConsulResult<AclPolicy>;

    /// This method reads an ACL policy with the given ID.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#read-a-policy
    async fn read_policy<S: AsRef<str> + Debug + Send>(
        &self,
        id: S,
    ) -> ConsulResult<Option<AclPolicy>>;

    /// This method reads an ACL policy with the given name.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#read-a-policy-by-name
    async fn read_policy_by_name<S: AsRef<str> + Debug + Send>(
        &self,
        name: S,
    ) -> ConsulResult<Option<AclPolicy>>;

    /// This method updates an existing ACL policy.
    ///
    ///	For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#update-a-policy
    async fn update_policy(&self, payload: UpdatePolicy) -> ConsulResult<AclPolicy>;

    /// This method deletes an ACL policy.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#delete-a-policy
    async fn delete_policy<S: AsRef<str> + Debug + Send>(&self, id: S) -> ConsulResult<bool>;

    /// This method lists all ACL policies.
    ///
    /// For more information, see the relevant endpoint's [API documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/acl/policies#list-policies
    async fn list_policies(&self) -> ConsulResult<Vec<AclPolicy>>;
}

#[async_trait]
impl AclPolicies for Client {
    async fn create_policy(&self, payload: CreatePolicy) -> ConsulResult<AclPolicy> {
        self.put("/v1/acl/policy", payload, None, None).await
    }

    async fn read_policy<S: AsRef<str> + Debug + Send>(
        &self,
        id: S,
    ) -> ConsulResult<Option<AclPolicy>> {
        self.get(format!("/v1/acl/policy/{}", id.as_ref()), None).await
    }

    async fn read_policy_by_name<S: AsRef<str> + Debug + Send>(
        &self,
        name: S,
    ) -> ConsulResult<Option<AclPolicy>> {
        self.get(format!("/v1/acl/policy/name/{}", name.as_ref()), None).await
    }

    async fn update_policy(&self, payload: UpdatePolicy) -> ConsulResult<AclPolicy> {
        self.put("/v1/acl/policy", payload, None, None).await
    }

    async fn delete_policy<S: AsRef<str> + Debug + Send>(&self, id: S) -> ConsulResult<bool> {
        self.delete(format!("/v1/acl/policy/{}", id.as_ref()), None, None).await
    }

    async fn list_policies(&self) -> ConsulResult<Vec<AclPolicy>> {
        self.get("/v1/acl/policies", None).await
    }
}
