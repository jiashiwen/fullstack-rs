use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Key, ReqScan, Response, KV};
use crate::httpserver::service::{s_raw_flush_all, s_raw_get, s_raw_put, s_raw_scan};
use axum::Json;
use axum_macros::debug_handler;

#[debug_handler]
pub async fn raw_put(Json(payload): Json<KV>) -> HandlerResult<()> {
    let result = s_raw_put(payload).await;
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

pub async fn raw_get(Json(payload): Json<Key>) -> HandlerResult<String> {
    let result = s_raw_get(payload.Key).await;
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

pub async fn raw_flush() -> HandlerResult<()> {
    let result = s_raw_flush_all().await;
    match result {
        Ok(()) => Ok(Json(Response::ok(()))),
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

pub async fn raw_scan(Json(req): Json<ReqScan>) -> HandlerResult<Vec<KV>> {
    let result = s_raw_scan(req.begin, req.end, req.limited).await;
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
