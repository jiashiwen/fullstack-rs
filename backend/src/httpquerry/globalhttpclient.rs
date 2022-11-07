use hyper::client::connect::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;

pub struct Http_Clients {
    pub http: hyper::Client<HttpConnector, Body>,
    pub https: hyper::Client<HttpsConnector<HttpConnector>, Body>,
}

impl Http_Clients {
    pub fn default() -> Self {
        let client = Client::new();
        let https = HttpsConnector::new();
        let client_https = Client::builder().build::<_, hyper::Body>(https);
        Self {
            http: client,
            https: client_https,
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_HTTP_CLIENT: Http_Clients = Http_Clients::default();
}
