mod config;
mod handler_login;
mod handler_rawkv;

mod authhandler;
mod handler_httpquery;
mod handler_redis;
mod handler_root;
mod handler_txn;
mod handler_user;

use axum::Json;
pub use config::current_config;
pub use handler_httpquery::baidu;
pub use handler_login::login;
pub use handler_login::logout;
pub use handler_rawkv::raw_flush;
pub use handler_rawkv::raw_get;
pub use handler_rawkv::raw_put;
pub use handler_rawkv::raw_scan;
pub use handler_redis::*;
pub use handler_root::root;
pub use handler_txn::{txn_get, txn_put};
pub use handler_user::get_headers;
pub use handler_user::get_user;
pub use handler_user::remove_user;
pub use handler_user::user_create;

use crate::httpserver::module::Response;

type HandlerResult<T> = crate::httpserver::module::Result<Json<Response<T>>>;
