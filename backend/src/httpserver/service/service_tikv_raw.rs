use crate::errors::{GlobalError, GlobalErrorType};
use crate::httpserver::module::KV;
use crate::resources::get_tikv_handler;
use anyhow::Error;
use anyhow::Result;

pub async fn s_raw_put(put: KV) -> Result<()> {
    let tikvhandler = get_tikv_handler().await;
    let result = tikvhandler.raw_put(put.Key, put.Value).await;
    match result {
        Ok(_) => Ok({}),
        Err(_e) => Err(anyhow::anyhow!("tikv error")),
    }
}

pub async fn s_raw_get(key: String) -> Result<String> {
    let tikvhandler = get_tikv_handler().await;
    let result = tikvhandler.raw_get(key).await.map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;

    match result {
        None => {
            return Err(Error::from(GlobalError::from_err(
                "no reault".to_string(),
                GlobalErrorType::UnknowErr,
            )));
        }
        Some(val) => {
            let str = String::from_utf8(val).map_err(|e| {
                return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
            })?;
            Ok(str)
        }
    }
}

pub async fn s_raw_flush_all() -> Result<()> {
    let tikvhandler = get_tikv_handler().await;
    tikvhandler.raw_remove_all().await.map_err(|e| {
        return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
    })?;
    Ok(())
}

pub async fn s_raw_scan(begin: String, end: String, limited: u32) -> Result<Vec<KV>> {
    let tikvhandler = get_tikv_handler().await;
    let result = tikvhandler
        .raw_scan(begin, end, limited)
        .await
        .map_err(|e| {
            return GlobalError::from_err(e.to_string(), GlobalErrorType::UnknowErr);
        })?;

    log::info!("{:?}", result);
    let mut kvarry = vec![];

    for pair in result {
        let key = pair.clone().into_key();
        let val = pair.clone().into_value();
        let key_str = String::from_utf8(Vec::from(key)).unwrap();
        let val_str = String::from_utf8(val).unwrap();

        kvarry.push(KV {
            Key: key_str,
            Value: val_str,
        });
    }

    Ok(kvarry)
}
