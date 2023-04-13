//! Common types and data structures used by the Consul API.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    #[serde(rename = "ID")]
    pub id: String,
    pub node: String,
    pub address: String,
    pub datacenter: String,
    pub tagged_addresses: HashMap<String, String>,
    pub meta: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaggedAddress {
    pub address: String,
    pub port: u16,
}
