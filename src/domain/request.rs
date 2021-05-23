use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hash: Option<BTreeMap<String, String>>,

    #[serde(rename = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_string: Option<String>,

    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_list: Option<Vec<String>>,

    #[serde(rename = "set")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set: Option<BTreeSet<String>>,

    #[serde(rename = "zset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_zset: Option<BTreeMap<String, f32>>,

    #[serde(skip_serializing)]
    pub(crate) ttl: usize,
}
