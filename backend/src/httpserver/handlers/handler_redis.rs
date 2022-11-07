use axum::Json;

use crate::httpserver::{
    exception::{AppError, AppErrorType},
    module::{Response, KV},
    service::put,
};

use super::HandlerResult;

pub async fn redis_put(Json(payload): Json<KV>) -> HandlerResult<()> {
    let result = put(payload);
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
