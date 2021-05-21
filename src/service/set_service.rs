use crate::adapters::repository::{RepoSet};
use crate::domain::request::Message;

use redis::RedisError;
use std::collections::BTreeSet;

use redis_cluster_async::{Connection,
                          redis::{cmd, Commands},
                          Client,
};
use redis_cluster_async::redis::aio::MultiplexedConnection;
pub async fn get_set(
    repo: Connection<MultiplexedConnection>,
    req: RepoSet,
) -> BTreeSet<String> {
    let resp = RepoSet::get(req.key, repo).await.unwrap();
    resp.value
}

pub async fn set_set(
    repo: Connection<MultiplexedConnection>,
    req: RepoSet,
) -> Result<(), RedisError> {
    let resp = RepoSet::set(req, repo).await;
    resp
}
pub  fn map_repo_set(k: String) -> RepoSet {
    RepoSet {
        value: Default::default(),
        key: k,
        ttl: 0,
    }
}
pub fn map_payload_to_repo_set(m: &Message, k: String) -> RepoSet {
    let v: BTreeSet<String> = m.m_set.clone().unwrap();

    RepoSet {
        value: v,
        key: k,
        ttl: m.ttl.clone(),
    }
}
