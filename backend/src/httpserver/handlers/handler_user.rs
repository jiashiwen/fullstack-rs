use crate::httpserver::exception::{AppError, AppErrorType};
use crate::httpserver::handlers::HandlerResult;
use crate::httpserver::module::{Response, User, UserName, ID};
use crate::httpserver::service::{s_get_user, s_remove_user, s_user_create};
use crate::privilege::{ActionType, ObjType, User as PrivilegeUser};

use crate::httpserver::handlers::authhandler::auth;
use axum::http::HeaderMap;
use axum::Json;

pub async fn get_headers(Json(id): Json<ID>, hm: HeaderMap) -> HandlerResult<()> {
    let ok = auth("global".to_string(), ObjType::User, ActionType::Create, hm).await;
    match ok {
        Ok(ok) => {
            if ok {
                println!("{:?}", id);
            }
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

    Ok(Json(Response::ok(())))
}

pub async fn user_create(Json(u): Json<User>) -> HandlerResult<()> {
    let r = s_user_create(u);
    match r {
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

pub async fn get_user(Json(u): Json<UserName>) -> HandlerResult<PrivilegeUser> {
    let user = s_get_user(u.name);
    match user {
        Ok(mut u) => {
            u.password = "".to_string();
            Ok(Json(Response::ok(u)))
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

pub async fn remove_user(Json(id): Json<ID>) -> HandlerResult<()> {
    let r = s_remove_user(id.id);
    match r {
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
