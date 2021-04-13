
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoList, RepoClient};

use crate::domain::request::{Message};
use redis::{ RedisError};

pub fn get_list(settings: &RedisConfig, req: RepoList) -> Vec<String>{
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoList::get(req.key,repo).unwrap();
    resp.value
}

pub fn set_list(settings: &RedisConfig, req: RepoList) -> Result<(), RedisError> {
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoList::set(req,repo);
    resp

}

pub fn map_repo_list( k:String) ->RepoList{
    RepoList{
        value: Default::default(),
        key: k,
        ttl: 0
    }
}
pub fn map_payload_to_repo_list(m: &Message, k:String) ->RepoList{
    let v:Vec<String>= m.m_list.clone().unwrap();

    RepoList{
        value: v,
        key: k,
        ttl: m.ttl.clone()
    }
}
