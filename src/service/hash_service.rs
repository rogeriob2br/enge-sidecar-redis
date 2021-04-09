use crate::adapters;
use crate::configs;
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoHash, RepoClient};
use std::collections::BTreeMap;

pub fn get_hash(settings: &RedisConfig, req: RepoHash) -> BTreeMap<String, String>{
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoHash::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_hash(settings: &RedisConfig, req: RepoHash) -> BTreeMap<String, String>{
    let repo: RepoClient = RepoClient::new(settings);
    let resp = RepoHash::set(req,repo).unwrap();
    resp.value
}