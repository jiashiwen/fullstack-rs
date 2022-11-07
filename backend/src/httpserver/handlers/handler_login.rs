use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Response, Token, User};
use crate::privilege::{gen_token, get_user_by_name, token_remove};
use axum::http::HeaderMap;
use axum::Json;

pub async fn login(Json(payload): Json<User>) -> HandlerResult<Token> {
    let user = get_user_by_name(payload.username.clone());

    match user {
        Ok(u) => {
            if payload.password.eq(u.password.as_str()) {
                let t = gen_token(u);
                return match t {
                    Ok(str) => {
                        let token = Token { token: str };
                        Ok(Json(Response::ok(token)))
                    }
                    Err(e) => {
                        let err = AppError {
                            message: Some(e.to_string()),
                            cause: None,
                            error_type: AppErrorType::UnknowErr,
                        };
                        Err(err)
                    }
                };
            }
            Ok(Json(Response::err(123, "password error".to_string())))
        }
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

pub async fn logout(hm: HeaderMap) -> HandlerResult<()> {
    let auth = hm.get("authorization");
    match auth {
        None => {
            let err = AppError {
                message: Some("no authorization".to_string()),
                cause: None,
                error_type: AppErrorType::UnknowErr,
            };
            return Err(err);
        }
        Some(val) => {
            token_remove(val.to_str().unwrap().to_string());
            Ok(Json(Response::ok(())))
        }
    }
}
