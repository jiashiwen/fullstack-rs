use crate::configure::{get_config, Config};
use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::Response;
use axum::Json;

pub async fn current_config() -> HandlerResult<Config> {
    let config = get_config();
    match config {
        Ok(cfg) => Ok(Json(Response::ok(cfg))),
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
