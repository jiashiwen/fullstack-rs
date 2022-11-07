use crate::{httpserver::module::KV, resources::GLOBAL_REDIS_POOL};
use anyhow::{anyhow, Result};

pub fn put(kv: KV) -> Result<()> {
    let conn = GLOBAL_REDIS_POOL.get();
    return match conn {
        Some(c) => {
            c.get()?
                .query(redis::cmd("set").arg(kv.Key).arg(kv.Value))?;
            Ok(())
        }
        None => Err(anyhow!("redis pool not init")),
    };
}
