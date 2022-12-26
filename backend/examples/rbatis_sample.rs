use std::fmt::Debug;

use rbatis::executor::Executor;
use rbatis::html_sql;
use rbatis::plugin::snowflake::new_snowflake_id;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql;
use rbatis::utils::impled;
use rbatis::Rbatis;
use rbatis::Result;
use rbdc_mysql::driver::MysqlDriver;
use rbs::to_value;
use rbs::Value;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "rbatis_sample";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub sex: Option<u8>,
    pub mobile: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
}

rbatis::crud!(BizActivity {}, TABLE_NAME);

pub async fn create(rb: &rbatis::Rbatis) -> Result<()> {
    let sql_statment = format!(
        "
    CREATE TABLE IF NOT EXISTS {} (
        id INT NOT NULL AUTO_INCREMENT,
        name VARCHAR(128) NOT NULL,
        sex TINYINT NOT NULL,
        mobile VARCHAR(11) NOT NULL,
        create_time DATETIME NOT NULL, 
        update_time DATETIME NOT NULL, 
        PRIMARY KEY(id)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        TABLE_NAME
    );
    rb.fetch_decode(&sql_statment, vec![]).await
}

#[html_sql("examples/rbatis_sample.html")]
async fn insert_rbatis_sample(rb: &mut dyn Executor, arg: Vec<Value>) -> Result<()> {
    impled!()
}

#[html_sql("examples/rbatis_sample.html")]
async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    dt: &FastDateTime,
) -> Vec<BizActivity> {
    impled!()
}

#[html_sql("examples/rbatis_sample.html")]
async fn update_by_condition(
    rb: &mut dyn Executor,
    id: u32,
    mobile: &str,
    name: &str,
    update_dt: &FastDateTime,
) -> Vec<()> {
    impled!()
}

// Todo html sql test
#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let mut rb = Rbatis::new();

    // 创建rbatis 实例
    rb.init(MysqlDriver {},"mysql://rust_test:Git785230root@mysql-internet-cn-north-1-1221449f8fb94332.rds.jdcloud.com:3306/rust_test")
        .unwrap();
    rb.get_pool().unwrap().resize(10);

    // 创建表
    println!("create table result: {:?}", create(&rb).await);
    println!("now : {}", FastDateTime::now());

    // 插入数据
    let bob = BizActivity {
        id: None,
        name: Some("Bob".to_string()),
        sex: Some(1),
        mobile: Some("13089977765".to_string()),
        create_time: Some(FastDateTime::now()),
        update_time: Some(FastDateTime::now()),
    };

    let jecika = BizActivity {
        id: None,
        name: Some("Jecika".to_string()),
        sex: Some(0),
        mobile: Some("130899734545".to_string()),
        create_time: Some(FastDateTime::now()),
        update_time: Some(FastDateTime::now()),
    };

    // insert
    let insert_bob = BizActivity::insert(&mut rb, &bob).await;
    let insert_jecika = BizActivity::insert(&mut rb, &jecika).await;

    println!("result insert bob: {:?}", insert_bob);
    println!("result insert insert_jecika: {:?}", insert_jecika);

    // count
    let sql_count = format!("select count(1) as count from {}", TABLE_NAME);
    let count: u64 = rb.fetch_decode(&sql_count, vec![]).await.unwrap();
    println!(">>>>> count={}", count);

    // select all
    let sql_select_all = format!("select * from {}", TABLE_NAME);
    let table: Vec<Option<BizActivity>> = rb.fetch_decode(&sql_select_all, vec![]).await.unwrap();
    println!(">>>>> table={:?}", table);

    //update
    let rs = update_by_condition(&mut rb, 1, "13097755678", "", &FastDateTime::now()).await;

    // select
    let sql_select_one = format!("select * from {} limit ?;", TABLE_NAME);
    let row: BizActivity = rb
        .fetch_decode(&sql_select_one, vec![to_value!(1)])
        .await
        .unwrap();
    println!(">>>>> row={:?}", row);

    // select by condition
    let data = select_by_condition(&mut rb, "Jecika", &FastDateTime::now()).await;
    println!("select by conditon result: {:?}", data);

    // select all
    let sql_select_all = format!("select * from {}", TABLE_NAME);
    let table: Vec<Option<BizActivity>> = rb.fetch_decode(&sql_select_all, vec![]).await.unwrap();
    println!(">>>>> table={:?}", table);

    // 清理数据
    let sql_trancate = format!("truncate table {}", TABLE_NAME);
    rb.fetch_decode::<()>(&sql_trancate, vec![]).await.unwrap();

    println!(">>>>> snowflake_id={:?}", new_snowflake_id().to_string());
}
