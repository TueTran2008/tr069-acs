use dioxus::prelude::server_fn::response;
//use dioxus::prelude::Context;
use kameo::{
    message::{Context, Message},
    Actor,
};
use tracing_log::log::info;

use crate::{
    cwmp_msg::{CWMPMsg, Envelope},
    session::cwmp_session::{CWMPSession, CWMPStateAction},
};

#[derive(Actor)]
pub struct DeviceActor {
    //Identity
    device_id: String,
    serial_number: String,
    manufacture_oui: String,
    product_class: String,

    // SessionID
    session_id: Option<String>,

    //handler:
    state: CWMPSession,
}

impl DeviceActor {
    pub fn new(session_id: String) -> Self {
        Self {
            device_id: String::new(),
            serial_number: String::new(),
            manufacture_oui: String::new(),
            product_class: String::new(),
            session_id: Some(session_id.clone()),
            state: CWMPSession::new(session_id.clone()),
        }
    }
}

impl Message<Envelope> for DeviceActor {
    type Reply = Envelope;

    async fn handle(
        &mut self,
        msg: Envelope,
        _ctx: &mut kameo::message::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        info!("Handle Envelope message {:?}", msg);
        let msg_type = msg.get_msg_type().unwrap();
        match msg_type {
            CWMPMsg::Inform(_content) => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveInform(
                        msg.get_msg_id().unwrap_or(&String::from("")).clone(),
                    ))
                    .await
                    .unwrap();
                return response.unwrap();
            }
            CWMPMsg::EmptyRPC => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveEmpty)
                    .await
                    .unwrap();
                return response.unwrap();
            }
            CWMPMsg::Fault(_) => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveResponse)
                    .await
                    .unwrap();
                return response.unwrap();
            }
            _ => todo!("Implement other"),
        }
    }
}
