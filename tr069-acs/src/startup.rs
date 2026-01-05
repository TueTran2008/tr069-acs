//use std::sync::Arc;

use std::sync::Arc;

use crate::{
    cwmp_msg::{self, session, Envelope, InformResponse},
    session::{consts::SESSION_KEY, SessionCwmp, SessionList},
};
//use crate::session;
use axum::{extract::State, Router};
//use axum_cookie::CookieManager;
//use axum_xml_up::Xml;
use tokio::{net::TcpListener, sync::RwLock};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
use uuid::Uuid;
// Global variable shared between thread and handler

struct AppState {
    app_session: Arc<RwLock<SessionList>>,
}
//#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    // dioxus::logger::initialize_default();

    use axum::routing::post;

    // use crate::cwmp_msg::session::{cwmp_session_handle, print_request_response};
    // let server_addr = SocketAddr::new(listener.local_addr());
    // tracing::info!("{server_addr}");
    //
    // //Build a custom router
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name(SESSION_KEY);

    //let shared_state = AppState {
    //    app_session: Arc::new(RwLock)
    //}
    let router = Router::new()
        .route("/", post(xml_request_handler))
        .layer(session_layer);
    //.with_state(state);
    // .layer(middleware::from_fn(print_request_response));
    axum::serve(listener, router).await.unwrap();
}

#[axum::debug_handler]
pub async fn xml_request_handler(
    //State(state): State<AppState>,
    session_id: Session,
    payload: Envelope,
) -> String {
    //tracing::info!("Get xml body: {:#?}", payload);
    //Retrieve a cookie
    //if let Some(cookie) = cookies.get("Session") {
    //    let span = tracing::span!(tracing::Level::DEBUG, "Handling request cwmp", Session_ID=%cookie.value());
    //} else {
    //    let session_uuid = Uuid::new_v4().simple().to_string();
    //    let session_cwmp = SessionCwmp::new(&session_uuid);
    //    tracing::debug!("New session created {}", session_uuid);
    //}
    //if let session_id = session_id.get(SESSION_KEY).await;
    tracing::debug!("Debug session {:#?}", session_id);
    if let Some(id) = session_id.id() {
        tracing::debug!("Get the session id {:?}", id);
    } else {
        let new_ssesion_id = Uuid::new_v4();
        let _ = session_id
            .insert("session", new_ssesion_id.to_string())
            .await;
    }
    //if let Ok(id) = session_id.get::<String>(SESSION_KEY).await {
    //    tracing::debug!(
    //        "Get the session id, maybe it is the Inform message: {:?}",
    //        id
    //    );
    //    session_id.insert(SESSION_KEY, "1").await.unwrap();
    //} else {
    //    tracing::debug!("Found no session ID");
    //    session_id.insert(SESSION_KEY, "2").await.unwrap();
    //}

    //SessionCwmp::handle_http(&mut self, cwmp_msg)
    let recv_msg_id = payload.get_msg_id().unwrap();

    let msg_body = InformResponse { max_envelopes: 1 };

    let res = Envelope::new(
        &String::from(recv_msg_id),
        cwmp_msg::CWMPMsg::InformResponse(msg_body),
    );
    //res.header.set_msg_id(recv_msg_id);

    let xml = String::from_utf8(res.create_xml().unwrap()).unwrap();

    tracing::info!("response {xml}");
    xml
}
