use serde::Deserialize;
use strum_macros::{Display, EnumString};

// #[derive(Deserialize)]
// pub struct CreateUser {
//     pub(crate) username: String,
// }

#[derive(EnumString, Display, Debug, PartialEq, Deserialize)]
pub enum Option {
    Put,
    Del,
    Get,
}

#[derive(Debug, Deserialize)]
pub struct ReqScan {
    pub begin: String,
    pub end: String,
    pub limited: u32,
}
