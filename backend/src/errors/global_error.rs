use std::fmt::Display;

/// 错误的类型
#[derive(Debug)]
pub enum GlobalErrorType {
    /// 未知错误
    UnknowErr,
}

/// 应用错误
#[derive(Debug)]
pub struct GlobalError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: GlobalErrorType,
}

impl GlobalError {
    /// 错误代码
    #[allow(dead_code)]
    fn code(&self) -> i32 {
        match self.error_type {
            GlobalErrorType::UnknowErr => 9999,
        }
    }
    /// 从上级错误中创建应用错误
    pub(crate) fn from_err(err: impl ToString, error_type: GlobalErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type: error_type,
        }
    }
    /// 从字符串创建应用错误
    #[allow(dead_code)]
    fn from_str(msg: &str, error_type: GlobalErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type: error_type,
        }
    }
    // /// 数据库错误
    // pub fn db_error(err: impl ToString) -> Self {
    //     Self::from_err(err, AppErrorType::DbError)
    // }
    // /// 未找到
    // pub fn not_found() -> Self {
    //     Self::from_str("不存在的记录", AppErrorType::NotFound)
    // }
}

impl std::error::Error for GlobalError {}

impl Display for GlobalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
