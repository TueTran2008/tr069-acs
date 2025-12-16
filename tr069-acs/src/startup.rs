use crate::cwmp_msg::{self, CWMPMsg, Envelope, InformResponse};
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
pub async fn xml_request_handler(payload: Envelope) -> String {
    tracing::info!("Get xml body: {:#?}", payload);

    let recv_msg_id = payload.get_msg_id();
    let msg_body = InformResponse { max_envelopes: 1 };
    let res = Envelope::new(recv_msg_id, cwmp_msg::CWMPMsg::InformResponse(msg_body));
    //res.header.set_msg_id(recv_msg_id);

    let xml = String::from_utf8(res.create_xml().unwrap()).unwrap();

    tracing::info!("response {xml}");
    xml
    //let extr_xml = Xml(xml);
    //tracing::info!("response message{:#?}", &extr_xml);
    //extr_xml
    // let xml_payload: Envelope = quick_xml::
}
