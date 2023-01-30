use sea_orm::ConnectionTrait;
use sea_orm::DbBackend;
use sea_orm::SqlxMySqlConnector;
use sea_orm::{FromQueryResult, Statement as sea_statment};
use sqlx::MySqlPool;

// pub const TABLE_NAME: &str = "sample";

#[derive(Debug, FromQueryResult)]
pub struct SeaOrmBiz {
    pub id: i64,
    pub name: String,
    pub gender: Option<i8>,
    pub mobile: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}

#[tokio::main]
async fn main() {
    let sqlx_opts = sqlx::mysql::MySqlConnectOptions::new()
        .host("gateway01.us-east-1.prod.aws.tidbcloud.com")
        .port(4000)
        .database("test")
        .username("2rxd6Puf9mpat5C.root")
        .password("gfBw8irgiHmAjsye")
        // .ssl_mode(sqlx_MysqlSslMode::VerifyIdentity)
        .ssl_ca("/etc/ssl/cert.pem");

    let pool = MySqlPool::connect_with(sqlx_opts).await.unwrap();
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(pool);

    let rs = db
        .execute(sea_statment::from_string(
            db.get_database_backend(),
            "select 1 from dual;".to_string(),
        ))
        .await;
    println!(">>>>> Cipher in use from sea_orm:{:?}", rs);

    let biz: Vec<SeaOrmBiz> = SeaOrmBiz::find_by_statement(sea_statment::from_sql_and_values(
        DbBackend::MySql,
        r#"SELECT * FROM sample; "#,
        vec![],
    ))
    .all(&db)
    .await
    .unwrap();
    println!(">>>>> selet rs is {:?}", biz);
}
