use clickhouse_rs::Pool;

#[tokio::main]
async fn main() {
    let database_url="tcp://sample:Git785230@service-terrabase-9s29mdlsb7.terrabase-9s29mdlsb7-hb-public.jvessel2.jdcloud.com:9000?compression=lz4".to_string();
    let pool = Pool::new(database_url);

    let mut client = pool.get_handle().await.unwrap();
    let sql = "show databases;";
    let r = client.query(sql).fetch_all().await;
    println!("result is: {:?}", r);
}
