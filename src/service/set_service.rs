
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoSet, RepoClient};
use std::collections::{BTreeMap, BTreeSet};
use crate::domain::request::{Message};
use redis::{ RedisError};

pub fn get_set(settings: &RedisConfig, req: RepoSet) -> BTreeSet<String>{
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoSet::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_set(settings: &RedisConfig, req: RepoSet) -> Result<(), RedisError> {
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoSet::set(req,repo);
    resp

}
pub fn map_repo_set(k:String) ->RepoSet{
    RepoSet{
        value: Default::default(),
        key: k,
        ttl: 0
    }
}
pub fn map_payload_to_repo_set(m: &Message, k:String) ->RepoSet{
    let v:BTreeSet<String>= m.m_set.clone().unwrap();

    RepoSet{
        value: v,
        key: k,
        ttl: m.ttl.clone()
    }
}