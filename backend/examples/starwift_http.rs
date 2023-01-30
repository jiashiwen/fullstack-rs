use clickhouse::Client;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
struct Database {
    name: String,
}

#[tokio::main]
async fn main() {
    let client = Client::default()
    .with_url("https://service-terrabase-9s29mdlsb7.terrabase-9s29mdlsb7-hb-public.jvessel2.jdcloud.com:8123")
    .with_user("sample")
    .with_password("Git785230");

    let sql = "SHOW databases";
    let r = client.query(sql).fetch_all::<Database>().await;
    println!("result is: {:?}", r);
}
