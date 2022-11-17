use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub col1: Option<i64>,
    pub col2: Option<String>,
}

#[tokio::main]
async fn main() {
    let rb = Rbatis::new();

    rb.init(MysqlDriver {},"mysql://rust_test:Git785230root@mysql-internet-cn-north-1-1221449f8fb94332.rds.jdcloud.com:3306/rust_test")
        .unwrap();
    rb.get_pool().unwrap().resize(10);

    let count: u64 = rb
        .fetch_decode("select count(1) as count from rbatis_t", vec![])
        .await
        .unwrap();

    println!(">>>>> count={}", count);
    let table: Vec<Option<BizActivity>> = rb
        .fetch_decode("select * from rbatis_t", vec![])
        .await
        .unwrap();

    println!(">>>>> table={:?}", table);
}
