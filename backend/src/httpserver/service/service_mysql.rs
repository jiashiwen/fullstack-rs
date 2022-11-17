use crate::{httpserver::dao::dao_test::RbatisTest, resources::GLOBAL_RBATIS_MYSQL};
use anyhow::Result;

pub async fn insert_rbatis_t() -> Result<()> {
    let data = RbatisTest {
        col1: Some(111 as i64),
        col2: Some("abcd".to_string()),
    };
    let mut rb = GLOBAL_RBATIS_MYSQL.clone();

    RbatisTest::insert(&mut rb, &data).await?;

    Ok(())
}
