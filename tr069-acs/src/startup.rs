use crate::cwmp_msg::session;
use dioxus::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

// Launch axum on the server
use axum::Router;
#[cfg(feature = "server")]
use tokio::runtime::Runtime;

#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    // dioxus::logger::initialize_default();

    use axum::{middleware, routing::post};

    use crate::cwmp_msg::session::{cwmp_session_handle, print_request_response};
    // let server_addr = SocketAddr::new(listener.local_addr());
    // tracing::info!("{server_addr}");
    //
    // //Build a custom router
    let router = Router::new()
        .route("/", post(cwmp_session_handle))
        .layer(middleware::from_fn(print_request_response));
    axum::serve(listener, router).await.unwrap();
}
