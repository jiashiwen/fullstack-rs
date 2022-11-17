use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

use crate::configure::get_config;
use anyhow::Result;

// 初始化 mysql 数据源
pub fn init_datasource_mysql() -> Result<Rbatis> {
    let config = get_config()?;
    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, &config.datasource_mysql.mysql_uri)?;
    let pool = rb.get_pool()?;
    pool.resize(config.datasource_mysql.pool_size);
    Ok(rb)
}
