use crate::httpquerry::globalhttpclient::GLOBAL_HTTP_CLIENT;
use crate::httpserver::module::Token;
use anyhow::Error;
use anyhow::Result;
use hyper::{Body, Method, Request};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

pub async fn query_login() -> Result<Option<Token>> {
    let req = Request::builder()
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .uri("http://127.0.0.1:3000/login")
        .body(Body::from(r#"{"username": "root",	"password": "123456"}"#))
        .expect("request builder");

    let resp = GLOBAL_HTTP_CLIENT.http.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    // let str = String::from_utf8(body_bytes.to_vec()).unwrap();
    // let str = serde_json::from_slice::<Value>(&*body_bytes);
    let data = serde_json::from_slice::<QueryResult<Token>>(&*body_bytes).map_err(|e| {
        return Error::new(e);
    })?;
    Ok(data.data)
}

pub async fn query_baidu() -> Result<String> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://www.baidu.com")
        .body(Body::empty())
        .expect("request builder");

    let resp = GLOBAL_HTTP_CLIENT.https.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let str = String::from_utf8(body_bytes.to_vec())?;

    Ok(str)
}
