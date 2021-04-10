use crate::adapters;
use crate::configs;
use crate::domain;
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoHash, RepoClient};
use std::collections::BTreeMap;
use crate::domain::request::{Message};
use actix_web::HttpResponse;
use redis::{RedisResult, RedisError};

pub fn get_hash(settings: &RedisConfig, req: RepoHash) -> BTreeMap<String, String>{
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoHash::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_hash(settings: &RedisConfig, req: RepoHash) -> Result<(), RedisError> {
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoHash::set(req,repo);
    resp

}

pub fn map_payload_to_repo_hash(m: &Message, k:String) ->RepoHash{
    let v:Option<BTreeMap<String,String>> = m.m_hash.clone();
    RepoHash{
        value: v.unwrap(),
        key: k,
        ttl: m.ttl.clone()
    }
}