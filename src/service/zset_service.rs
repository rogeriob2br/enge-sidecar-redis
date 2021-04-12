
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoZSet, RepoClient};
use std::collections::BTreeMap;
use crate::domain::request::{Message};
use redis::{ RedisError};

pub fn get_zset(settings: &RedisConfig, req: RepoZSet) -> BTreeMap<String, f32>{
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoZSet::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_zset(settings: &RedisConfig, req: RepoZSet) -> Result<(), RedisError> {
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoZSet::set(req,repo);
    resp

}
pub fn map_repo_zset(k:String) ->RepoZSet{
    RepoZSet{
        value: Default::default(),
        key: k,
        ttl: 0
    }
}
pub fn map_payload_to_repo_zset(m: &Message, k:String) ->RepoZSet{
    let v:BTreeMap<String, f32>= m.m_zset.clone().unwrap();

    RepoZSet{
        value: v,
        key: k,
        ttl: m.ttl.clone()
    }
}