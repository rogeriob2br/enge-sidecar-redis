use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap, BTreeSet};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message{
    #[serde(rename = "hash")]
    pub m_hash: Option<BTreeMap<String, String>>,

    #[serde(rename = "string")]
    pub m_string: Option<String>,

    #[serde(rename = "List")]
    pub m_list: Option<Vec<String>>,

    #[serde(rename = "set")]
    pub m_set: Option<BTreeSet<String>>,

    #[serde(rename = "zset")]
    pub m_zset: Option<BTreeMap<String, f32>>,

    pub(crate) ttl: usize
}