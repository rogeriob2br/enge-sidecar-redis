use crate::adapters::repository::{RepoString};
use crate::domain::request::Message;

use redis::RedisError;
use redis_cluster_async::{Connection,
                          redis::{cmd, Commands},
                          Client,
};
use redis_cluster_async::redis::aio::MultiplexedConnection;

pub async fn get_string(
    repo: Connection<MultiplexedConnection>,
    req: RepoString,
) -> String {
    let resp = RepoString::get(req.key, repo).await.unwrap();
    resp.value
}

pub async fn set_string(
    repo: Connection<MultiplexedConnection>,
    req: RepoString,
) -> Result<(), RedisError> {
    let resp = RepoString::set(req, repo).await;
    resp
}
pub fn map_repo_string(k: String) -> RepoString {
    RepoString {
        value: Default::default(),
        key: k,
        ttl: 0,
    }
}
pub fn map_payload_to_repo_string(m: &Message, k: String) -> RepoString {
    let v: String = m.m_string.clone().unwrap();

    RepoString {
        value: v,
        key: k,
        ttl: m.ttl.clone(),
    }
}
