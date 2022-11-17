use crate::resources::tikv::TiKVHandler;
use anyhow::Result;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use rbatis::Rbatis;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use super::mysql_rbatis_resource::init_datasource_mysql;
use super::redis_resource::gen_redis_conn_pool;
use super::redis_resource::RedisConnectionManager;

// lazy_static::lazy_static! {
//     static ref GLOBAL_PD_ENDPOINT: Mutex<Vec<String>> = Mutex::new(vec![]);

//     static ref GLOBAL_TiKV: AsyncOnce<TiKVHandler> = AsyncOnce::new(async {
//         let endpoint= GLOBAL_PD_ENDPOINT.lock().unwrap().to_vec();
//         let pd: Vec<&str> = endpoint.iter().map(|s| &**s).collect();
//         let global_TiKV = TiKVHandler::new(pd).await;
//         global_TiKV
//     });
// }
// 全局 redis pool
pub static GLOBAL_REDIS_POOL: OnceCell<r2d2::Pool<RedisConnectionManager>> = OnceCell::new();
pub static GLOBAL_RBATIS_MYSQL: Lazy<rbatis::Rbatis> = Lazy::new(|| {
    let rb = match init_datasource_mysql() {
        Ok(rb) => rb,
        Err(err) => panic!("{}", err),
    };
    rb
});

static GLOBAL_PD_ENDPOINT: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static GLOBAL_TiKV: Lazy<TiKVHandler> = Lazy::new(|| {
    let endpoint = GLOBAL_PD_ENDPOINT.lock().unwrap().to_vec();
    let pd: Vec<&str> = endpoint.iter().map(|s| &**s).collect();
    let global_TiKV = futures::executor::block_on(async { TiKVHandler::new(pd).await });
    global_TiKV
});

pub struct DataSourceMysql {
    pub rbatis: Rbatis,
}

pub async fn init_resources() -> Result<()> {
    // let cfg = get_config()?;
    // let pd: Vec<&str> = cfg.tikv.pdaddrs.iter().map(|s| &**s).collect();
    // //配置tikv
    // set_tikv(pd);
    // //tikv连接初始化
    // let rawclient = get_tikv_handler();
    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    //     rawclient.await.raw_get("t".to_string()).await;
    // });
    // let txnclient = get_tikv_handler();
    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    //     txnclient
    //         .await
    //         .txn_put("1".to_string(), "1".to_string())
    //         .await;
    // });
    init_global_redis();
    init_global_rbatis_mysql().await?;
    Ok(())
}

fn init_global_redis() {
    GLOBAL_REDIS_POOL.get_or_init(|| {
        let pool = match gen_redis_conn_pool() {
            Ok(it) => it,
            Err(err) => panic!("{}", err.to_string()),
        };
        pool
    });
}

async fn init_global_rbatis_mysql() -> Result<()> {
    let _ = GLOBAL_RBATIS_MYSQL
        .exec("select 1 from dual", vec![])
        .await?;
    Ok(())
}

pub fn set_tikv(endpoint: Vec<&str>) {
    if endpoint.is_empty() {
        GLOBAL_PD_ENDPOINT
            .lock()
            .unwrap()
            .push("127.0.0.1:2379".to_string());
        return;
    }

    for str in endpoint {
        GLOBAL_PD_ENDPOINT.lock().unwrap().push(String::from(str));
    }
}

pub async fn get_tikv_handler() -> &'static TiKVHandler {
    &GLOBAL_TiKV
}
