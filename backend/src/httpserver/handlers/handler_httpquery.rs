use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::Response;

use crate::httpquerry::query_baidu;
use axum::http::HeaderMap;
use axum::Json;

pub async fn baidu(hm: HeaderMap) -> HandlerResult<String> {
    let res = query_baidu().await;

    match res {
        Err(e) => {
            log::error!("{}", e);
            let err = AppError {
                message: Some("querry error".to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
        Ok(val) => Ok(Json(Response::ok(val))),
    }
}
