mod common_module;
mod request_module;
mod response_module;

pub use common_module::Key;
pub use common_module::Token;
pub use common_module::User;
pub use common_module::UserName;
pub use common_module::ID;
pub use common_module::KV;

pub use request_module::ReqScan;
pub use response_module::Response;

/// 定义自己的 Result
pub type Result<T> = std::result::Result<T, crate::httpserver::exception::AppError>;
