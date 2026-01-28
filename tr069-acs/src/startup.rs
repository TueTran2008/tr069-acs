//use std::sync::Arc;

use std::sync::Arc;

use crate::{
    cwmp_msg::{self, session, Envelope, InformResponse},
    session::{
        consts::{SESSION_EXPIRE_TIME, SESSION_KEY},
        SessionCwmp, SessionList,
    },
};
//use crate::session;
use axum::routing::post;
use axum::Router;

//use axum_cookie::CookieManager;
//use axum_xml_up::Xml;
use tokio::{net::TcpListener, sync::RwLock};
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, Session, SessionManagerLayer};
use uuid::Uuid;
// Global variable shared between thread and handler

struct AppState {
    app_session: Arc<RwLock<SessionList>>,
}
//#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    let session_store = MemoryStore::default();
    let session_expire = Expiry::OnInactivity(Duration::seconds(SESSION_EXPIRE_TIME as i64));
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(session_expire)
        .with_name(SESSION_KEY);

    let router = Router::new()
        .route("/", post(xml_request_handler))
        .layer(session_layer);

    axum::serve(listener, router).await.unwrap();
}

#[axum::debug_handler]
pub async fn xml_request_handler(
    //State(state): State<AppState>,
    session: Session,
    payload: Envelope,
) -> String {
    let session_id = session.get::<String>("session_id").await;

    match session_id {
        Ok(session_id) => {
            if let Some(session_id) = session_id {
                tracing::debug!("Found the session ID in the request {:?}", session_id);
                todo!("Implement when session ID available");
            } else {
                let new_ssesion_id = Uuid::new_v4();
                let _get_id = session
                    .insert("session_id", &new_ssesion_id.to_string())
                    .await;
                tracing::debug!("Generate new session_id {:?}", new_ssesion_id.to_string());
            }
        }

        Err(_e) => {
            todo!("Implement when cannot get the session with the Json and Store errors");
        }
    }
    //match id {}

    tracing::debug!("Get the session id {:?} - {:#?}", session, session);

    //if let Some(id) = session.id() {
    //    //let found_id = session_id.get::<String>(&id.to_string()).await;
    //    tracing::debug!("Found the session ID in the request {:?}", id);
    //} else {
    //}

    //let res = SessionCwmp::handle_http(&mut self, cwmp_msg);
    let recv_msg_id = payload.get_msg_id().unwrap();

    let msg_body = InformResponse { max_envelopes: 1 };

    let res = Envelope::new(
        &String::from(recv_msg_id),
        cwmp_msg::CWMPMsg::InformResponse(msg_body),
    );

    let xml = String::from_utf8(res.create_xml().unwrap()).unwrap();

    tracing::info!("response {xml}");
    xml
}
