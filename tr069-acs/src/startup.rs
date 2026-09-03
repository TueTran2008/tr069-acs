use crate::{
    cwmp_msg::{CWMPMsg, Envelope},
    session::{
        consts::{SESSION_EXPIRE_TIME, SESSION_KEY},
        device_actor::DeviceActor,
        device_manager::DeviceSessionManager,
    },
};
use axum::{extract::State, routing::post};
use axum::{response::IntoResponse, Router};
use kameo::actor::ActorRef;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, Session, SessionManagerLayer};
use tracing_log::log::info;
use uuid::Uuid;
// Global variable shared between thread and handler
#[derive(Clone)]
pub struct AppState {
    //app_session: Arc<RwLock<SessionList>>,
    device_manager: Arc<RwLock<DeviceSessionManager>>,
}

//#[cfg(feature = "server")]
pub async fn run(listener: TcpListener) {
    let state = AppState {
        //app_session: Arc::new(RwLock::new(SessionList::default())),
        device_manager: Arc::new(RwLock::new(DeviceSessionManager::new())),
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
) -> impl IntoResponse {
    let session_id = session.get::<String>("session_id").await;
    let device: ActorRef<DeviceActor>;
    match session_id {
        Ok(session_id) => {
            if let Some(session_id) = session_id {
                //todo!("Implement when session ID available");
                device = state.device_manager.read().await.get(&session_id).unwrap();
            } else {
                let new_session_id = Uuid::new_v4().to_string();
                {
                    let mut manager = state.device_manager.write().await;
                    manager.insert_session(new_session_id.clone());
                }
                tracing::debug!("Generate new session_id {:?}", new_session_id.to_string());
                device = state
                    .device_manager
                    .read()
                    .await
                    .get(&new_session_id)
                    .unwrap();
            }
        }

        Err(_e) => {
            todo!("Implement when cannot get the session with the Json and Store errors");
        }
    }
    info!("Before asking to Kameo {:?}", payload);
    let xml = device.ask(payload).await.unwrap();
    info!("After asking to Kameo");
    info!("{:?}", xml);
    xml
}
