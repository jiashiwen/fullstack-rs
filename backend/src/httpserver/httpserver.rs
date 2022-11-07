use crate::httpserver::routers::router_root;
use axum::Router;
use std::net::{self, SocketAddr};
use tokio::spawn;
use tokio::sync::oneshot::Receiver;
use tokio::task::JoinHandle;

pub struct HttpServer {
    pub addr: SocketAddr,
    pub router: Router,
}

impl HttpServer {
    pub fn default() -> Self {
        let port: u16 = 3000;
        let addr_ipv4 = net::SocketAddr::from((net::Ipv4Addr::UNSPECIFIED, port));

        let band_addr = SocketAddr::from(addr_ipv4);
        let app = router_root();
        Self {
            addr: band_addr,
            router: app,
        }
    }

    pub async fn run(&mut self, rx: Receiver<()>) -> JoinHandle<()> {
        let server = axum::Server::bind(&self.addr)
            .serve(self.router.clone().into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok();
            });

        let handle = spawn(async {
            server.await.unwrap();
        });
        log::info!("httpserver start");
        return handle;
    }
}
