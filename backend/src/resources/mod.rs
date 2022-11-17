mod embed_resources;
mod init_resources;
mod mysql_rbatis_resource;
mod redis_resource;
mod tikv;

pub use embed_resources::get_rbac_model;
pub use embed_resources::get_rbac_policy;
pub use init_resources::get_tikv_handler;
pub use init_resources::init_resources;
pub use init_resources::set_tikv;

pub use init_resources::GLOBAL_RBATIS_MYSQL;
pub use init_resources::GLOBAL_REDIS_POOL;
pub use mysql_rbatis_resource::init_datasource_mysql;
pub use redis_resource::InstanceType;
pub use redis_resource::RedisInstance;
pub use tikv::TiKVHandler;
