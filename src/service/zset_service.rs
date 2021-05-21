use crate::adapters::repository::{RepoZSet};
use crate::domain::request::Message;

use redis::RedisError;
use std::collections::BTreeMap;
use redis_cluster_async::{Connection,
                          redis::{cmd, Commands},
                          Client,
};
use redis_cluster_async::redis::aio::MultiplexedConnection;

pub async fn get_zset(
    repo: Connection<MultiplexedConnection>,
    req: RepoZSet,
) -> BTreeMap<String, f32> {
    let resp = RepoZSet::get(req.key, repo).await.unwrap();
    resp.value
}

pub async fn set_zset(
    repo: Connection<MultiplexedConnection>,
    req: RepoZSet,
) -> Result<(), RedisError> {
    let resp = RepoZSet::set(req, repo).await;
    resp
}
pub fn map_repo_zset(k: String) -> RepoZSet {
    RepoZSet {
        value: Default::default(),
        key: k,
        ttl: 0,
    }
}
pub fn map_payload_to_repo_zset(m: &Message, k: String) -> RepoZSet {
    let v: BTreeMap<String, f32> = m.m_zset.clone().unwrap();

    RepoZSet {
        value: v,
        key: k,
        ttl: m.ttl.clone(),
    }
}
