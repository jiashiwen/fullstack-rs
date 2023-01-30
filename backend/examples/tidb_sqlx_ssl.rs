use futures::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    let sqlx_opts = sqlx::mysql::MySqlConnectOptions::new()
        .host("gateway01.us-east-1.prod.aws.tidbcloud.com")
        .port(4000)
        .database("test")
        .username("2rxd6Puf9mpat5C.root")
        .password("gfBw8irgiHmAjsye")
        // .ssl_mode(MySqlSslMode::VerifyIdentity)
        .ssl_ca("/etc/ssl/cert.pem");

    let pool = MySqlPoolOptions::new()
        .connect_with(sqlx_opts)
        .await
        .unwrap();

    // let r = sqlx::query("SELECT 150").execute(&pool).await;
    let mut rows = sqlx::query("select * from sample").fetch(&pool);
    while let Some(row) = rows.try_next().await.unwrap() {
        println!("row is {:?}", row);
    }
}
