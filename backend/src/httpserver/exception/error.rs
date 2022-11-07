//! 自定义错误
use std::fmt::Display;

use axum::{response::IntoResponse, Json};

use crate::httpserver::module::Response;

/// Error type
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppErrorType {
    /// 未知错误
    UnknowErr,
    /// 数据库错误
    DbError,
    /// 未找到
    NotFound,
}

/// 应用错误
#[derive(Debug)]
pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}

impl AppError {
    /// 错误代码
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::DbError => 1,
            AppErrorType::NotFound => 2,
            AppErrorType::UnknowErr => 9999,
        }
    }
    /// 从上级错误中创建应用错误
    #[allow(dead_code)]
    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }
    /// 从字符串创建应用错误
    #[allow(dead_code)]
    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }
    /// 数据库错误
    #[allow(dead_code)]
    pub fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbError)
    }
    /// 未找到
    #[allow(dead_code)]
    pub fn not_found() -> Self {
        Self::from_str("不存在的记录", AppErrorType::NotFound)
    }
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<apperror::AppError> for AppError {
//     fn from(apperror: AppError) -> Self {
//         apperror
//     }
// }

// impl From<deadpool_postgres::PoolError> for AppError {
//     fn from(err: deadpool_postgres::PoolError) -> Self {
//         Self::db_error(err)
//     }
// }
//
// impl From<tokio_postgres::Error> for AppError {
//     fn from(err: tokio_postgres::Error) -> Self {
//         Self::db_error(err)
//     }
// }

/// 实现 IntoResponse
impl IntoResponse for AppError {
    // type Body = Full<Bytes>;
    // type BodyError = Infallible;

    // fn into_response(self) -> axum::http::Response<Self::Body> {
    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = match self.message {
            Some(msg) => msg,
            None => "有错误发生".to_string(),
        };
        let res: Response<()> = Response::err(code, msg);
        Json(res).into_response()
    }
}
