use std::fmt::Debug;
use std::str::FromStr;

use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Local;
use rbatis::executor::Executor;
use rbatis::html_sql;
use rbatis::plugin::snowflake::new_snowflake_id;

use rbatis::rbdc::datetime::DateTime as RbatisDateTime;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::snowflake;
use rbatis::sql;
use rbatis::utils::impled;
use rbatis::Rbatis;
use rbatis::Result;
use rbdc_mysql::driver::MysqlDriver;
use rbs::to_value;

use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "rbatis_sample";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub gender: Option<u8>,
    pub mobile: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
}

rbatis::crud!(BizActivity {}, TABLE_NAME);

// 创建表
pub async fn create(rb: &rbatis::Rbatis) -> Result<()> {
    let sql_statment = format!(
        "
    CREATE TABLE IF NOT EXISTS {} (
        id BIGINT NOT NULL ,
        name VARCHAR(128) NOT NULL,
        gender TINYINT NOT NULL,
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
async fn insert_rbatis_sample(rb: &mut dyn Executor, arg: Vec<String>) -> Result<()> {
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
    update_dt: &RbatisDateTime,
) -> Result<()> {
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
        id: Some(new_snowflake_id()),
        name: Some("Bob".to_string()),
        gender: Some(1),
        mobile: Some("13089977765".to_string()),
        create_time: Some(FastDateTime::now()),
        update_time: Some(FastDateTime::now()),
    };

    let jecika = BizActivity {
        id: Some(new_snowflake_id()),
        name: Some("Jecika".to_string()),
        gender: Some(0),
        mobile: Some("130899734545".to_string()),
        create_time: Some(FastDateTime::now()),
        update_time: Some(FastDateTime::now()),
    };

    // insert
    let insert_bob = BizActivity::insert(&mut rb, &bob).await;
    let insert_jecika = BizActivity::insert(&mut rb, &jecika).await;

    println!("result insert bob: {:?}", insert_bob);
    println!("result insert insert_jecika: {:?}", insert_jecika);

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string();
    // 2021-01-04 20:02:09
    // insert by html
    // let arg = vec![
    //     to_value!(new_snowflake_id()),
    //     to_value!("Rex".to_string()),
    //     to_value!("1".to_string()),
    //     to_value!("13098899876".to_string()),
    //     to_value!(FastDateTime::now()),
    //     to_value!(FastDateTime::now()),
    // ];
    let arg = vec![
        r#"'"#.to_owned() + &new_snowflake_id().to_string() + r#"'"#,
        r#"""#.to_owned() + &"Rex".to_string() + r#"""#,
        "1".to_string(),
        r#"""#.to_owned() + &"13098899876".to_string() + r#"""#,
        r#"'"#.to_owned() + &str_date + r#"'"#,
        r#"'"#.to_owned() + &str_date + r#"'"#,
    ];

    // let arg = vec![
    //     new_snowflake_id().to_string(),
    //     "Rex".to_string(),
    //     "1".to_string(),
    //     "13098899876".to_string(),
    //     str_date.clone(),
    //     str_date.clone(),
    // ];
    insert_rbatis_sample(&mut rb, arg).await.unwrap();

    // count
    let sql_count = format!("select count(1) as count from {}", TABLE_NAME);
    let count: u64 = rb.fetch_decode(&sql_count, vec![]).await.unwrap();
    println!(">>>>> count={}", count);

    // select all
    let sql_select_all = format!("select * from {}", TABLE_NAME);
    let table = rb
        .fetch_decode::<Vec<Option<BizActivity>>>(&sql_select_all, vec![])
        .await;
    println!(">>>>> table={:?}", table);

    //update
    // let rs = update_by_condition(&mut rb, 1, "13097755678", "", &FastDateTime::now()).await;
    let rs = update_by_condition(
        &mut rb,
        1,
        "13097755678",
        "",
        &RbatisDateTime::from_str(&"2022-12-27 11:53:00").unwrap(),
    )
    .await;

    println!(">>>>> update by condition result: {:?}", rs);

    // select
    let sql_select_one = format!("select * from {} limit ?;", TABLE_NAME);
    let row = rb
        .fetch_decode::<BizActivity>(&sql_select_one, vec![to_value!(1)])
        .await;
    println!(">>>>> row={:?}", row);

    // select by condition
    let data = select_by_condition(&mut rb, "Jecika", &FastDateTime::now()).await;
    println!(">>>>> select by conditon result: {:?}", data);

    // select all
    let sql_select_all = format!("select * from {}", TABLE_NAME);
    let table = rb
        .fetch_decode::<Vec<Option<BizActivity>>>(&sql_select_all, vec![])
        .await;
    println!(">>>>> table={:?}", table);

    // 清理数据
    // let sql_trancate = format!("truncate table {}", TABLE_NAME);
    // rb.fetch_decode::<()>(&sql_trancate, vec![]).await.unwrap();

    // println!(">>>>> snowflake_id={:?}", new_snowflake_id().to_string());
}
