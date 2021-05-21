use crate::adapters::repository::RepoHash;
use crate::domain::request::Message;
use redis::RedisError;
use std::collections::BTreeMap;
use redis_cluster_async::{Connection,
    redis::{cmd, Commands},
    Client,
};
use redis_cluster_async::redis::aio::MultiplexedConnection;

pub async fn get_hash(
    repo: Connection<MultiplexedConnection>,
    req: RepoHash,
) -> BTreeMap<String, String> {
    let resp = RepoHash::get(req.key, repo).await.unwrap();

    resp.value
}

pub async fn set_hash(
    repo: Connection<MultiplexedConnection>,
    req: RepoHash,
) -> Result<(), RedisError> {
    let resp = RepoHash::set(req, repo).await;
    resp
}
pub fn map_repo_hash(k: String) -> RepoHash {
    RepoHash {
        value: Default::default(),
        key: k,
        ttl: 0,
    }
}
pub fn map_payload_to_repo_hash(m: &Message, k: String) -> RepoHash {
    let v: BTreeMap<String, String> = m.m_hash.clone().unwrap();

    RepoHash {
        value: v,
        key: k,
        ttl: m.ttl.clone(),
    }
}
