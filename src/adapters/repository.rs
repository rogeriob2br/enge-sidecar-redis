
use mobc::{Pool};
use mobc_redis_cluster::RedisClusterConnectionManager;
use mobc_redis_cluster::{redis, Connection};
use redis::RedisResult;
use redis_cluster_async::{ redis::{cmd},
};

use std::collections::{BTreeMap, BTreeSet};

pub type MobcPool = Pool<RedisClusterConnectionManager>;


pub struct RepoHash {
    pub value: BTreeMap<String, String>,
    pub key: String,
    pub ttl: usize,
}

async fn get_con(pool: &MobcPool) -> mobc::Connection<RedisClusterConnectionManager> {
    pool.get().await.unwrap()
}

impl RepoHash {
    pub async fn set(data: RepoHash, pool: &MobcPool) -> RedisResult<()> {
        let mut con = get_con(&pool).await;

        cmd("HMSET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut con as &mut Connection).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut con as &mut Connection).await?;
        }

        Ok(())
    }
    pub async fn get(key: String, pool: &MobcPool) -> RedisResult<RepoHash> {
        let mut con = get_con(&pool).await;
        let info: BTreeMap<String, String> = cmd("HGETALL").arg(&key).query_async(&mut con as &mut Connection).await?;
        Ok(RepoHash {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoString {
    pub value: String,
    pub key: String,
    pub ttl: usize,
}

impl RepoString {
    pub async fn set(data: RepoString, pool: &MobcPool) -> RedisResult<()> {
        let mut con = get_con(&pool).await;
        cmd("SET")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut con as &mut Connection).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut con as &mut Connection).await?;
        }

        Ok(())
    }
    pub async fn get(
        key: String,
        pool: &MobcPool,
    ) -> RedisResult<RepoString> {
        let mut con = get_con(&pool).await;

        let info: String = cmd("GET").arg(&key).query_async(&mut con as &mut Connection).await?;
        Ok(RepoString {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoList {
    pub value: Vec<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoList {
    pub async fn set(data: RepoList, pool: &MobcPool) -> RedisResult<()> {
        let mut con = get_con(&pool).await;
        cmd("RPUSH")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut con as &mut Connection).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut con as &mut Connection).await?;
        }

        Ok(())
    }
    pub async fn get(key: String, pool: &MobcPool) -> RedisResult<RepoList> {
        let mut con = get_con(&pool).await;

        let info: Vec<String> = cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query_async(&mut con as &mut Connection).await.unwrap();
        Ok(RepoList {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoSet {
    pub value: BTreeSet<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoSet {
    pub async fn set(data: RepoSet, pool: &MobcPool) -> RedisResult<()> {
        let mut con = get_con(&pool).await;
        cmd("SADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut con as &mut Connection).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut con as &mut Connection).await?;
        }
        Ok(())
    }
    pub async fn get(key: String, pool: &MobcPool) -> RedisResult<RepoSet> {
        let mut con = get_con(&pool).await;
        let info: BTreeSet<String> = cmd("SMEMBERS").arg(&key).query_async(&mut con as &mut Connection).await?;
        Ok(RepoSet {
            key,
            value: info,
            ttl: 0,
        })
    }
}

pub struct RepoZSet {
    pub value: BTreeMap<String, f32>,
    pub key: String,
    pub ttl: usize,
}

impl RepoZSet {
    pub async fn set(data: RepoZSet, pool: &MobcPool) -> RedisResult<()> {
        let mut con = get_con(&pool).await;
        cmd("ZADD")
            .arg(data.key.clone())
            .arg(data.value.clone())
            .query_async(&mut con as &mut Connection).await?;

        if data.ttl > 0 {
            cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query_async(&mut con as &mut Connection).await?;
        }
        Ok(())
    }
    pub async fn get(key: String, pool: &MobcPool) -> RedisResult<RepoZSet> {
        let mut con = get_con(&pool).await;
        let info: BTreeMap<String, f32> = cmd("ZRANGE")
            .arg(&key)
            .arg("-inf")
            .arg("+inf")
            .arg("WITHSCORES")
            .query_async(&mut con as &mut Connection).await?;

        Ok(RepoZSet {
            key,
            value: info,
            ttl: 0,
        })
    }
}
