use redis::{Commands, RedisResult, FromRedisValue};
use crate::configs::reader_cfg::RedisConfig;
use redis::cluster::{ ClusterClient};
use std::collections::{BTreeMap, BTreeSet};
use serde::de::Unexpected::Str;

pub struct RepoClient{
    pub db: ClusterClient,

}
impl RepoClient{
    pub fn new(settings: &RedisConfig) -> RepoClient{
        let nodes = &settings.redis_uris;
        let client = ClusterClient::open(nodes.clone()).unwrap();
        let repo: RepoClient = RepoClient{
            db: client,
        };
        repo
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
        let _: () = redis::cmd("HMSET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)
            .expect("failed to execute HMSET");
        let x:usize = 0;
        if data.ttl.clone() > x{
            let _: ()  = redis::cmd("EXPIRE")
                .arg(data.key.clone())
                .arg(data.ttl.clone())
                .query(&mut conn)
                .expect("failed to execute EXPIRE");
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoHash>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let mut info: BTreeMap<String, String> = redis::cmd("HGETALL")
            .arg(&key)
            .query(&mut conn)
            .expect("failed to execute HGETALL");


        let result: RepoHash = RepoHash{
            key: key,
            value: info.clone(),
            ttl: 0
        };
        Ok(result)
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
        let _: () = redis::cmd("SET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)
            .expect("failed to execute SET");
        let x:usize = 0;
        if data.ttl.clone() > x{
            let _: ()  = redis::cmd("EXPIRE")
                .arg(data.key.clone())
                .arg(data.ttl.clone())
                .query(&mut conn)
                .expect("failed to execute EXPIRE");
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoString>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let mut info: String= redis::cmd("GET")
            .arg(&key)
            .query(&mut conn)
            .expect("failed to execute GET");


        let result: RepoString = RepoString{
            key: key,
            value: info.clone(),
            ttl: 0
        };
        Ok(result)
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
        let _: () = redis::cmd("RPUSH")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)
            .expect("failed to execute RPUSH");
        let x:usize = 0;
        if data.ttl.clone() > x{
            let _: ()  = redis::cmd("EXPIRE")
                .arg(data.key.clone())
                .arg(data.ttl.clone())
                .query(&mut conn)
                .expect("failed to execute EXPIRE");
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoList>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let mut info: Vec<String>= redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut conn)
            .expect("failed to execute GET");


        let result: RepoList = RepoList{
            key: key,
            value: info.clone(),
            ttl: 0
        };
        Ok(result)
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
        let _: () = redis::cmd("SADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)
            .expect("failed to execute SADD");
        let x:usize = 0;
        if data.ttl.clone() > x{
            let _: ()  = redis::cmd("EXPIRE")
                .arg(data.key.clone())
                .arg(data.ttl.clone())
                .query(&mut conn)
                .expect("failed to execute EXPIRE");
        }
        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoSet>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let mut info: BTreeSet<String>= redis::cmd("SMEMBERS")
            .arg(&key)
            .query(&mut conn)
            .expect("failed to execute GET");


        let result: RepoSet = RepoSet{
            key: key,
            value: info.clone(),
            ttl: 0
        };
        Ok(result)
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
        let _: () = redis::cmd("ZADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query(&mut conn)
            .expect("failed to execute ZADD");
        let x:usize = 0;
        if data.ttl.clone() > x{
            let _: ()  = redis::cmd("EXPIRE")
                .arg(data.key.clone())
                .arg(data.ttl.clone())
                .query(&mut conn)
                .expect("failed to execute EXPIRE");
        }
        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoZSet>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let mut info: BTreeMap<String, f32>= redis::cmd("ZRANGE")
            .arg(&key)
            .arg("-inf")
            .arg("+inf")
            .arg("WITHSCORES")
            .query(&mut conn)
            .expect("failed to execute GET");


        let result: RepoZSet = RepoZSet{
            key,
            value: info.clone(),
            ttl: 0
        };
        Ok(result)
    }

}
