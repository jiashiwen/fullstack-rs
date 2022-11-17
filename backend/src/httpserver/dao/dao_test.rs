use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RbatisTest {
    pub col1: Option<i64>,
    pub col2: Option<String>,
}

rbatis::crud!(RbatisTest {}, "rbatis_t");
