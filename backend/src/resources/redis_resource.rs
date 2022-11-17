use std::time::Duration;

use crate::configure::get_config;
use anyhow::Result;

use r2d2::Pool;
use redis::{
    cluster::ClusterClientBuilder, from_redis_value, ConnectionLike, FromRedisValue, RedisError,
    RedisResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InstanceType {
    Single,
    Cluster,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisInstance {
    #[serde(default = "RedisInstance::urls_default")]
    pub urls: Vec<String>,
    #[serde(default = "RedisInstance::password_default")]
    pub password: String,
    #[serde(default = "RedisInstance::instance_type_default")]
    pub instance_type: InstanceType,
}

impl Default for RedisInstance {
    fn default() -> Self {
        Self {
            urls: vec!["redis://127.0.0.1:6379".to_string()],
            password: "".to_string(),
            instance_type: InstanceType::Single,
        }
    }
}

impl RedisInstance {
    pub fn urls_default() -> Vec<String> {
        vec!["redis://127.0.0.1:6379".to_string()]
    }
    pub fn password_default() -> String {
        "".to_string()
    }
    pub fn instance_type_default() -> InstanceType {
        InstanceType::Single
    }

    pub fn to_redis_client(&self) -> RedisResult<RedisClient> {
        return match self.instance_type {
            InstanceType::Single => {
                let cl = redis::Client::open(self.urls[0].as_str())?;
                Ok(RedisClient::Single(cl))
            }
            InstanceType::Cluster => {
                let mut cb = ClusterClientBuilder::new(self.urls.clone());
                if !self.password.is_empty() {
                    cb = cb.password(self.password.clone());
                }
                let cl = cb.build()?;
                Ok(RedisClient::Cluster(cl))
            }
        };
    }
}

#[derive(Clone)]
pub enum RedisClient {
    Single(redis::Client),
    Cluster(redis::cluster::ClusterClient),
}

impl RedisClient {
    pub fn get_redis_connection(&self) -> RedisResult<RedisConnection> {
        return match self {
            RedisClient::Single(s) => {
                let conn = s.get_connection()?;
                Ok(RedisConnection::Single(Box::new(conn)))
            }
            RedisClient::Cluster(c) => {
                let conn = c.get_connection()?;
                Ok(RedisConnection::Cluster(Box::new(conn)))
            }
        };
    }
}

pub enum RedisConnection {
    Single(Box<redis::Connection>),
    Cluster(Box<redis::cluster::ClusterConnection>),
}

impl RedisConnection {
    pub fn is_open(&self) -> bool {
        return match self {
            RedisConnection::Single(sc) => sc.is_open(),
            RedisConnection::Cluster(cc) => cc.is_open(),
        };
    }

    pub fn query<T: FromRedisValue>(&mut self, cmd: &redis::Cmd) -> RedisResult<T> {
        return match self {
            RedisConnection::Single(sc) => match sc.as_mut().req_command(cmd) {
                Ok(val) => from_redis_value(&val),
                Err(e) => Err(e),
            },
            RedisConnection::Cluster(cc) => match cc.req_command(cmd) {
                Ok(val) => from_redis_value(&val),
                Err(e) => Err(e),
            },
        };
    }
}

#[derive(Clone)]
pub struct RedisConnectionManager {
    pub redis_client: RedisClient,
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = RedisConnection;
    type Error = RedisError;

    fn connect(&self) -> Result<RedisConnection, Self::Error> {
        let conn = self.redis_client.get_redis_connection()?;
        Ok(conn)
    }

    fn is_valid(&self, conn: &mut RedisConnection) -> Result<(), Self::Error> {
        match conn {
            RedisConnection::Single(sc) => {
                redis::cmd("PING").query(sc)?;
            }
            RedisConnection::Cluster(cc) => {
                redis::cmd("PING").query(cc)?;
            }
        }
        Ok(())
    }

    fn has_broken(&self, conn: &mut RedisConnection) -> bool {
        !conn.is_open()
    }
}

// pub fn init_redis_pool() {
//     GLOBAL_REDIS_POOL.get_or_init(|| {
//         let pool = gen_redis_conn_pool().unwrap();
//         pool
//     });
// }

pub fn gen_redis_conn_pool() -> Result<Pool<RedisConnectionManager>> {
    let config = get_config()?;
    let redis_client = config.redis.instance.to_redis_client()?;
    let manager = RedisConnectionManager { redis_client };
    let pool = r2d2::Pool::builder()
        .max_size(config.redis.pool.max_size as u32)
        .min_idle(Some(config.redis.pool.mini_idle as u32))
        .connection_timeout(Duration::from_secs(
            config.redis.pool.connection_timeout as u64,
        ))
        .build(manager)?;
    Ok(pool)
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::{RedisClient, RedisConnectionManager};

    //cargo test resources::redis_resource::test::test_redis_connection_manager --  --nocapture
    #[test]
    fn test_redis_connection_manager() {
        println!("test_redis_connection_manager");
        let client = redis::Client::open("redis://:redistest0102@114.67.76.82:16377").unwrap();
        let rc = RedisClient::Single(client);

        let manager = RedisConnectionManager { redis_client: rc };
        let pool = r2d2::Pool::builder()
            .max_size(6)
            .min_idle(Some(1))
            .build(manager)
            .unwrap();

        let mut handles = vec![];

        for _i in 0..10i32 {
            let pool = pool.clone();
            handles.push(thread::spawn(move || {
                let mut conn = pool.get().unwrap();
                let rs = conn.query::<String>(&redis::cmd("ping"));
                println!("{:?}", rs);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
        println!("{}", pool.state().idle_connections);
    }
}
