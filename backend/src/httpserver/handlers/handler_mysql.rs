use axum::Json;

use crate::httpserver::{
    exception::{AppError, AppErrorType},
    module::Response,
    service::insert_rbatis_t,
};

use super::HandlerResult;

pub async fn rbatis_t_insert() -> HandlerResult<()> {
    let result = insert_rbatis_t().await;
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
