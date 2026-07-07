use crate::{
    cwmp_msg::{CWMPMsg, Envelope},
    session::{
        consts::{SESSION_EXPIRE_TIME, SESSION_KEY},
        device_manager::DeviceSessionManager,
    },
};
use axum::Router;
use axum::{extract::State, routing::post};
use tokio::net::TcpListener;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, Session, SessionManagerLayer};
use uuid::Uuid;
// Global variable shared between thread and handler
#[derive(Clone)]
pub struct AppState {
    //app_session: Arc<RwLock<SessionList>>,
    device_manager: DeviceSessionManager,
}

//#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    let state = AppState {
        //app_session: Arc::new(RwLock::new(SessionList::default())),
        device_manager: DeviceSessionManager::new(),
    };
    let session_store = MemoryStore::default();
    let session_expire = Expiry::OnInactivity(Duration::seconds(SESSION_EXPIRE_TIME as i64));
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(session_expire)
        .with_name(SESSION_KEY);

    let router = Router::new()
        .route("/", post(xml_request_handler))
        .layer(session_layer)
        .with_state(state);

    axum::serve(listener, router).await.unwrap();
}

#[axum::debug_handler]
pub async fn xml_request_handler(
    State(state): State<AppState>,
    session: Session,
    payload: Envelope,
) -> String {
    let session_id = session.get::<String>("session_id").await;

    match session_id {
        Ok(session_id) => {
            if let Some(session_id) = session_id {
                todo!("Implement when session ID available");
            } else {
                let new_ssesion_id = Uuid::new_v4();
                let _get_id = session
                    .insert("session_id", &new_ssesion_id.to_string())
                    .await;

                tracing::debug!("Generate new session_id {:?}", new_ssesion_id.to_string());

                if let Some(CWMPMsg::Inform(inform)) = payload.get_msg_body() {
                } else {
                    tracing::error!("First message should be inform");
                }
            }
        }

        Err(_e) => {
            todo!("Implement when cannot get the session with the Json and Store errors");
        }
    }

    let xml = String::from("Danzel dumfries");

    tracing::info!("response {xml}");
    xml
}
