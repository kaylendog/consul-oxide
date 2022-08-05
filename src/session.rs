use async_trait::async_trait;

use crate::{sealed::Sealed, Client, ConsulResult, QueryOptions};

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SessionID {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SessionEntry {
    #[serde(rename = "CreateIndex")]
    pub createindex: Option<u64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Node")]
    pub node: Option<String>,
    #[serde(rename = "LockDelay")]
    pub lockdelay: Option<u64>, //delay: Change this to a Durations
    #[serde(rename = "Behavior")]
    pub behavior: Option<String>,
    #[serde(rename = "Checks")]
    pub checks: Option<Vec<String>>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

#[async_trait]
pub trait Session: Sealed {
    /// This method initializes a new session. Sessions must be associated
    /// with a node and may be associated with any number of checks.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#create-session
    async fn create_session(
        &self,
        session: SessionEntry,
        options: Option<QueryOptions>,
    ) -> ConsulResult<SessionEntry>;

    /// This method destroys the session with the given name. If the session
    /// UUID is malformed, an error is returned. If the session UUID does not
    /// exist or already expired, `true` is still returned (the operation is
    /// idempotent).
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#delete-session
    async fn destroy_session(&self, id: &str, options: Option<QueryOptions>) -> ConsulResult<bool>;

    /// This method returns the requested session information.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#read-session
    async fn get_session_info(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;

    /// This endpoint returns the list of active sessions.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#list-sessions
    async fn list_sessions(&self, options: Option<QueryOptions>)
        -> ConsulResult<Vec<SessionEntry>>;

    /// This method method returns the active sessions for a given node.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#list-sessions-for-node
    async fn list_session_for_node(
        &self,
        node: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;

    /// This method renews the given session. This should be used with sessions
    /// that have a TTL, and it extends the expiration by the TTL.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/session#renew-session
    async fn renew_session(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>>;
}

#[async_trait]
impl Session for Client {
    #[tracing::instrument]
    async fn create_session(
        &self,
        session: SessionEntry,
        options: Option<QueryOptions>,
    ) -> ConsulResult<SessionEntry> {
        self.put("/v1/session/create", session, None, options).await
    }

    #[tracing::instrument]
    async fn destroy_session(&self, id: &str, options: Option<QueryOptions>) -> ConsulResult<bool> {
        let path = format!("/v1/session/destroy/{}", id);
        self.put(&path, None as Option<&()>, None, options).await
    }

    #[tracing::instrument]
    async fn get_session_info(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/info/{}", id);
        self.get(&path, options).await
    }

    #[tracing::instrument]
    async fn list_sessions(
        &self,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        self.get("/v1/session/list", options).await
    }

    #[tracing::instrument]
    async fn list_session_for_node(
        &self,
        node: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/node/{}", node);
        self.get(&path, options).await
    }

    #[tracing::instrument]
    async fn renew_session(
        &self,
        id: &str,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<SessionEntry>> {
        let path = format!("/v1/session/renew/{}", id);
        self.put(&path, None as Option<&()>, None, options).await
    }
}
