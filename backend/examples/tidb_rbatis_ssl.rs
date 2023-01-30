use mysql::prelude::*;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::Rbatis;
use rbdc_mysql::options::MySqlConnectOptions;
use rbdc_mysql::{driver::MysqlDriver, options::MySqlSslMode as rbdc_MysqlSslMode};
use rbs::to_value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TABLE_NAME: &str = "sample";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizRbatis {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub gender: Option<u8>,
    pub mobile: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
}
rbatis::crud!(BizRbatis {}, TABLE_NAME);

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // rbatis 连接
    let rb = Rbatis::new();
    let opt = MySqlConnectOptions::new()
        .host("gateway01.us-east-1.prod.aws.tidbcloud.com")
        .port(4000)
        .database("test")
        .username("2rxd6Puf9mpat5C.root")
        .password("gfBw8irgiHmAjsye")
        .ssl_mode(rbdc_MysqlSslMode::VerifyIdentity)
        .ssl_ca("/etc/ssl/cert.pem");
    rb.init_opt(MysqlDriver {}, opt).unwrap();
    rb.get_pool().unwrap().resize(3);

    let sql_show_ssl_cipher = "SHOW STATUS LIKE 'Ssl_cipher'";
    let cipher_rbatis = rb
        .fetch_decode::<Vec<rbs::Value>>(sql_show_ssl_cipher, vec![])
        .await;
    println!(">>>>> Cipher in use from rbatis: {:?}", cipher_rbatis);

    let sql_select_one = format!("select * from {} limit ?;", TABLE_NAME);
    let row = rb
        .fetch_decode::<BizRbatis>(&sql_select_one, vec![to_value!(1)])
        .await;
    println!(">>>>> rbatsis select result={:?}", row);

    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let mut my_rb = Rbatis::new();

    // 创建rbatis 实例
    my_rb.init(MysqlDriver {},"mysql://rust_test:Git785230root@mysql-internet-cn-north-1-1221449f8fb94332.rds.jdcloud.com:3306/rust_test")
        .unwrap();
    my_rb.get_pool().unwrap().resize(10);

    let my_cipher_rbatis = my_rb
        .fetch_decode::<Vec<rbs::Value>>(sql_show_ssl_cipher, vec![])
        .await;
    println!(
        ">>>>> Mysql Cipher in use from rbatis: {:?}",
        my_cipher_rbatis
    );

    Ok(())
}
