
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoString, RepoClient};
use crate::domain::request::{Message};
use redis::{ RedisError};

pub fn get_string(repo: &RepoClient, req: RepoString) -> String{

    let resp = RepoString::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_string(repo: &RepoClient, req: RepoString) -> Result<(), RedisError> {

    let resp = RepoString::set(req,repo);
    resp

}
pub fn map_repo_string( k:String) ->RepoString{
    RepoString{
        value: Default::default(),
        key: k,
        ttl: 0
    }
}
pub fn map_payload_to_repo_string(m: &Message, k:String) ->RepoString{
    let v:String= m.m_string.clone().unwrap();

    RepoString{
        value: v,
        key: k,
        ttl: m.ttl.clone()
    }
}