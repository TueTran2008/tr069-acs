pub mod consts;
//use std::collections::HashMap;

//use reqwest::header::ValuesMut;
mod cwmp_session;
pub mod device_actor;
pub mod device_manager;
pub mod state;
use crate::cwmp_msg::{self, get_names::GetParamterNames, CWMPMsg, InformResponse};

/*
* Initialize a session in case there is a new inform
*/
enum VersionCwmp {
    Version1_4,
    Version1_3,
    Version1_2,
    Version1_1,
}

//Represent the ACS session state
//There should be  4 state in the ACS cwmp Session
enum SessionState {
    // Session Initialize, Wait for an Inform
    Idle,
    // This state is after the ACS sent the Inform Response to CPE,
    SentInformResponse,
    // This can happen multiple time
    WaitCpeRequest,
    // This can happen multiple time
    SendRpcResponse,

    Termination,
}

pub struct SessionCwmp {
    //Represent the session ID
    // Cwmp verison used in the session
    version: VersionCwmp,
    // Current state of session
    state: SessionState,
    //// Session id
    session_id: Option<String>,
}
//Contains a list a ses(0sion spawn by TR-069
//struct Session {
//    // Store the Session ID and Session State
//    session_map
//}
// Use Option for CWMPMsg and Envelope in Response because they could be empty

impl SessionCwmp {
    pub fn new(id: &str) -> Self {
        Self {
            session_id: Some(String::from(id)),
            state: SessionState::Idle,
            version: VersionCwmp::Version1_4,
        }
    }

    // This sequence should be apply to new Device
    pub fn handle_http(&mut self, cwmp_msg: Option<CWMPMsg>) -> Option<CWMPMsg> {
        match self.state {
            // In the idle state, only start the session if it is Inform
            SessionState::Idle => {
                if let Some(CWMPMsg::Inform(inform)) = cwmp_msg {
                    self.state = SessionState::SentInformResponse;
                    let msg_body = InformResponse::default();
                    let res = cwmp_msg::CWMPMsg::InformResponse(msg_body);
                    Some(res)
                } else {
                    None
                }
            }

            SessionState::SentInformResponse => {
                if let Some(CWMPMsg::EmptyRPC) = cwmp_msg {
                    let msg_body = GetParamterNames::new("Device.".to_string(), true);
                    let res = cwmp_msg::CWMPMsg::GetParameterNames(msg_body);
                    Some(res)
                } else {
                    None
                }
            }
            _ => todo!("Implementing other"),
        }
    }
}

impl Default for SessionCwmp {
    fn default() -> Self {
        Self {
            version: VersionCwmp::Version1_4,
            state: SessionState::Idle,
            session_id: None,
        }
    }
}

#[derive(Default)]
pub struct DeviceIndentify {
    serial_number: String,
    product_class: Option<String>,
}

#[derive(Default)]
pub struct SessionList {
    session: DeviceIndentify,
}

impl SessionList {
    pub fn insert_sn(mut self, value: String) {
        let new_device = DeviceIndentify {
            serial_number: value,
            product_class: None,
        };
        //self.session.push(new_device);
    }
}

//impl Default for SessionList {
//    fn default() -> Self {
//        Self {
//            session: Vec::new(),
//        }
//    }
//}
