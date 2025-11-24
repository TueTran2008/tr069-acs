use crate::cwmp_msg::{self, CWMPMsg, Envelope, InformResponse};
use axum::{response, Router};
use axum_xml_up::Xml;
use tokio::net::TcpListener;
// use axum::response::S
use yaserde;

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
pub async fn xml_request_handler(envelope: String) {
    tracing::info!("Get xml body: {:?}", envelope);
    // match envelope.get_rpc_type() {
    //     _ => {
    //         tracing::info!("Got message type");
    //     }
    // }
    // let response = InformResponse::default();
    // let envelope_response = Envelope::new(CWMPMsg::InformResponse(response));
    let xml_payload: Envelope = yaserde::de::from_str(&envelope).unwrap();
    tracing::info!("response xml {:?}", xml_payload);
    // Xml(envelope_response)
}
