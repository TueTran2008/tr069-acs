use std::sync::Arc;

//use dioxus::prelude::Context;
use crate::{
    ctx::Ctx,
    cwmp_msg::{CWMPMsg, Envelope},
    model::{
        device::{DeviceBmc, DeviceForCreate},
        ModelManager,
    },
    session::cwmp_session::{CWMPSession, CWMPStateAction},
};
use kameo::{message::Message, Actor};
use tracing::info;
use tracing::info_span;

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
    mm: Arc<ModelManager>,
}

impl DeviceActor {
    pub fn new(session_id: String, mm: ModelManager) -> Self {
        Self {
            device_id: String::new(),
            serial_number: String::new(),
            manufacture_oui: String::new(),
            product_class: String::new(),
            session_id: Some(session_id.clone()),
            state: CWMPSession::new(session_id.clone()),
            mm: Arc::new(mm),
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
        info_span!("");
        let msg_type = msg.get_msg_type().unwrap();
        match msg_type {
            CWMPMsg::Inform(content) => {
                // Create new Device into Database or Up
                let db_context = Ctx::new(1).unwrap();
                info!(
                    sn = %content.get_sn().unwrap(),
                    message = ?msg,
                    //"Handle CWMP message"
                );

                //let a = content.get_sn().unwrap().to_string();
                let device_create = DeviceForCreate::new(
                    content.get_sn().unwrap().clone(),
                    content.get_oui().map(|x| x.to_string()),
                    content.get_product_class().map(|x| x.to_string()),
                    content.get_manufacture().map(|x| x.to_string()),
                );
                //let l_mm = Arc::clone(&self.mm);
                let _device_dbc =
                    DeviceBmc::create(&db_context, Arc::clone(&self.mm).as_ref(), device_create)
                        .await;
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveInform(
                        msg.get_msg_id().unwrap_or(&String::from("")).clone(),
                    ))
                    .await
                    .unwrap();
                response.unwrap()
            }
            CWMPMsg::EmptyRPC => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveEmpty)
                    .await
                    .unwrap();
                response.unwrap()
            }
            CWMPMsg::Fault(_) => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveResponse)
                    .await
                    .unwrap();
                response.unwrap()
            }
            CWMPMsg::GetParameterNames(names) => {
                info!("GPN response {:#?}", names);
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveResponse)
                    .await
                    .unwrap();
                response.unwrap()
            }
            _ => {
                let response = self
                    .state
                    .apply_action(CWMPStateAction::CWMPStateReceiveResponse)
                    .await
                    .unwrap();
                response.unwrap()
            }
        }
    }
}
