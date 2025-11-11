use crate::cwmp_msg::{self};
use axum::Router;
use axum_xml_up::Xml;
use tokio::net::TcpListener;

#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    // dioxus::logger::initialize_default();

    use axum::{middleware, routing::post};

    // use crate::cwmp_msg::session::{cwmp_session_handle, print_request_response};
    // let server_addr = SocketAddr::new(listener.local_addr());
    // tracing::info!("{server_addr}");
    //
    // //Build a custom router
    let router = Router::new().route("/", post(xml_request_handler));
    // .layer(middleware::from_fn(print_request_response));
    axum::serve(listener, router).await.unwrap();
}

#[axum::debug_handler]
pub async fn xml_request_handler(Xml(payload): Xml<cwmp_msg::Envelope>) {
    tracing::info!("Get xml body: {:?}", payload);
}
