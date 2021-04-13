use redis::{ RedisResult};
use crate::configs::reader_cfg::RedisConfig;
use redis::cluster::{ ClusterClient};
use std::collections::{BTreeMap, BTreeSet};

pub struct RepoClient{
    pub db: ClusterClient,

}
impl RepoClient{
    pub fn new(settings: &RedisConfig) -> RedisResult<ClusterClient> {
        let nodes = &settings.redis_uris;
        ClusterClient::open(nodes.clone())

    }
}

pub struct RepoHash{
    pub value: BTreeMap<String, String>,
    pub key: String,
    pub ttl: usize,
}
impl RepoHash {
    pub fn set(data: RepoHash, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
       redis::cmd("HMSET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)?;

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoHash>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: BTreeMap<String, String> = redis::cmd("HGETALL")
            .arg(&key)
            .query(&mut conn)?;
        Ok(RepoHash{
            key,
            value: info,
            ttl: 0
        })
    }

}

pub struct RepoString{
    pub value: String,
    pub key: String,
    pub ttl: usize,
}

impl RepoString {
    pub fn set(data: RepoString, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        redis::cmd("SET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)?;

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoString>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: String= redis::cmd("GET")
            .arg(&key)
            .query(&mut conn)?;
        Ok(RepoString{
            key,
            value: info,
            ttl: 0
        })
    }

}

pub struct RepoList{
    pub value: Vec<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoList {
    pub fn set(data: RepoList, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        redis::cmd("RPUSH")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)?;

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoList>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: Vec<String>= redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut conn)?;
        Ok(RepoList{
            key,
            value: info,
            ttl: 0
        })
    }

}


pub struct RepoSet{
    pub value: BTreeSet<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoSet {
    pub fn set(data: RepoSet, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        redis::cmd("SADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)?;

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }
        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoSet>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: BTreeSet<String>= redis::cmd("SMEMBERS")
            .arg(&key)
            .query(&mut conn)?;
        Ok(RepoSet{
            key,
            value: info,
            ttl: 0
        })
    }

}


pub struct RepoZSet{
    pub value: BTreeMap<String, f32>,
    pub key: String,
    pub ttl: usize,
}

impl RepoZSet {
    pub fn set(data: RepoZSet, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        redis::cmd("ZADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)?;

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }
        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoZSet>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: BTreeMap<String, f32>= redis::cmd("ZRANGE")
            .arg(&key)
            .arg("-inf")
            .arg("+inf")
            .arg("WITHSCORES")
            .query(&mut conn)?;

        Ok(RepoZSet{
            key,
            value: info,
            ttl: 0
        })
    }

}
