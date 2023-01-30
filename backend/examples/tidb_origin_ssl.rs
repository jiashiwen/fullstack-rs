use chrono::Local;
use mysql::prelude::*;
use mysql::*;
use rbatis::snowflake::new_snowflake_id;
use serde::Deserialize;
use serde::Serialize;

pub const TABLE_NAME: &str = "sample";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizOrigin {
    pub id: i64,
    pub name: String,
    pub gender: u8,
    pub mobile: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let fmt = "%Y-%m-%d %H:%M:%S";
    // 原生方式连接
    let cert_path = std::path::Path::new("/etc/ssl/cert.pem");
    let ssl_opts = SslOpts::default().with_root_cert_path(Some(cert_path));
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("gateway01.us-east-1.prod.aws.tidbcloud.com"))
        .tcp_port(4000)
        .user(Some("2rxd6Puf9mpat5C.root"))
        .pass(Some("gfBw8irgiHmAjsye"))
        .ssl_opts(ssl_opts)
        .db_name(Some("test"));

    let mut conn_origin = Conn::new(opts)?;
    let (_, cipher_origin): (Value, String) = "SHOW STATUS LIKE 'Ssl_cipher'"
        .first(&mut conn_origin)?
        .unwrap();
    println!(">>>>> Cipher in use from origin: {}", cipher_origin);

    let create_statment = format!(
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
    conn_origin.query_drop(create_statment)?;

    let bizes = vec![
        BizOrigin {
            id: new_snowflake_id(),
            name: "Bob".to_string(),
            gender: 1,
            mobile: "13037777876".to_string(),
            create_time: Some(Local::now().format(fmt).to_string()),
            update_time: Some(Local::now().format(fmt).to_string()),
        },
        BizOrigin {
            id: new_snowflake_id(),
            name: "Jecika".to_string(),
            gender: 0,
            mobile: "13033457876".to_string(),
            create_time: Some(Local::now().format(fmt).to_string()),
            update_time: Some(Local::now().format(fmt).to_string()),
        },
    ];

    conn_origin.exec_batch(
        r"insert into sample (id,name,gender,mobile,create_time,update_time) 
    values (:id,:name,:gender,:mobile,:create,:update)",
        bizes.iter().map(|p| -> Params {
            params! {
                "id"=>p.id,
                "name"=>p.name.to_owned(),
                "gender"=>p.gender.to_owned(),
                "mobile"=>p.mobile.to_owned(),
                "create"=>p.create_time.as_ref(),
                "update"=>p.update_time.as_ref()
            }
        }),
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_bizs = conn_origin.query_map(
        "SELECT id,name,gender,mobile,create_time,update_time from sample",
        |(id, name, gender, mobile, create_time, update_time)| BizOrigin {
            id,
            name,
            gender,
            mobile,
            create_time,
            update_time,
        },
    )?;
    println!("selected result {:?}", selected_bizs);

    Ok(())
}
