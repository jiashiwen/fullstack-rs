use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Key, ReqScan, Response, KV};
use crate::httpserver::service::{s_txn_get, s_txn_put};
use axum::Json;

pub async fn txn_put(Json(payload): Json<KV>) -> HandlerResult<()> {
    let result = s_txn_put(payload).await;
    log::info!("handle raw_put result: {:?}", result);
    match result {
        Ok(_) => Ok(Json(Response::ok(()))),
        Err(e) => {
            let err = AppError {
                message: Some(e.to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
    }
}

pub async fn txn_get(Json(payload): Json<Key>) -> HandlerResult<String> {
    let result = s_txn_get(payload.Key).await;
    match result {
        Ok(str) => Ok(Json(Response::ok(str))),
        Err(e) => {
            let err = AppError {
                message: Some(e.to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
    }
}

// pub async fn txn_flush() -> HandlerResult<()> {
//     let result = s_raw_flush_all().await;
//     match result {
//         Ok(()) => Ok(Json(Response::ok(()))),
//         Err(e) => {
//             let err = AppError {
//                 message: Some(e.to_string()),
//                 cause: None,
//                 error_type: AppErrorType::UnknowErr,
//             };
//             return Err(err);
//         }
//     }
// }
//
// pub async fn txn_scan(Json(req): Json<ReqScan>) -> HandlerResult<Vec<KV>> {
//     let result = s_raw_scan(req.begin, req.end, req.limited).await;
//     match result {
//         Ok(str) => Ok(Json(Response::ok(str))),
//         Err(e) => {
//             let err = AppError {
//                 message: Some(e.to_string()),
//                 cause: None,
//                 error_type: AppErrorType::UnknowErr,
//             };
//             return Err(err);
//         }
//     }
// }
