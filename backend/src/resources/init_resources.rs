use crate::configure::get_config;
use crate::resources::tikv::TiKVHandler;
use anyhow::Result;
use async_once::AsyncOnce;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GLOBAL_PD_ENDPOINT: Mutex<Vec<String>> = Mutex::new(vec![]);

    static ref GLOBAL_TiKV: AsyncOnce<TiKVHandler> = AsyncOnce::new(async {
        let endpoint= GLOBAL_PD_ENDPOINT.lock().unwrap().to_vec();
        let pd: Vec<&str> = endpoint.iter().map(|s| &**s).collect();
        let global_TiKV = TiKVHandler::new(pd).await;
        global_TiKV
    });
}

pub fn init_resources() -> Result<()> {
    let cfg = get_config()?;
    let pd: Vec<&str> = cfg.tikv.pdaddrs.iter().map(|s| &**s).collect();
    //配置tikv
    set_tikv(pd);
    //tikv连接初始化
    let rawclient = get_tikv_handler();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        rawclient.await.raw_get("t".to_string()).await;
    });
    let txnclient = get_tikv_handler();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        txnclient.await.txn_put("1".to_string(), "1".to_string()).await;
    });

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
    GLOBAL_TiKV.get().await
}
