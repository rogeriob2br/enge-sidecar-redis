use crate::adapters::repository::{RepoList};

use crate::domain::request::Message;

use redis::RedisError;
use redis_cluster_async::{Connection,
                          redis::{cmd, Commands},
                          Client,
};
use redis_cluster_async::redis::aio::MultiplexedConnection;


pub async fn get_list(
    repo: Connection<MultiplexedConnection>,
    req: RepoList,
) -> Vec<String> {
    let resp = RepoList::get(req.key, repo).await.unwrap();
    resp.value
}

pub async fn set_list(
    repo: Connection<MultiplexedConnection>,
    req: RepoList,
) -> Result<(), RedisError> {
    let resp = RepoList::set(req, repo).await;
    resp
}

pub fn map_repo_list(k: String) -> RepoList {
    RepoList {
        value: Default::default(),
        key: k,
        ttl: 0,
    }
}
pub fn map_payload_to_repo_list(m: &Message, k: String) -> RepoList {
    let v: Vec<String> = m.m_list.clone().unwrap();

    RepoList {
        value: v,
        key: k,
        ttl: m.ttl.clone(),
    }
}
