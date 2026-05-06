//pub struct StateIdle;
//
use crate::error::Result;
use crate::{
    cwmp_msg::{CWMPMsg, Envelope, InformResponse},
    session::state::client_state::ClientState,
};

pub struct StateIdle; // When recevied Inform message from CWMP Client
pub struct StateExchangeRpc;
pub struct StateNoMoreRpc;

pub struct CWMPSession {
    response_envelope: Option<Envelope>,
    device_id: String,
    list_rpc: Vec<CWMPMsg>,
    state: Box<dyn ClientState + Send + Sync>,
}

impl CWMPSession {
    pub fn new(device_id: String) -> Self {
        Self {
            response_envelope: None,
            device_id,
            list_rpc: Vec::new(),
            state: Box::new(StateIdle),
        }
    }
    pub fn set_response_rpc(&mut self, envelope: Envelope) {
        self.response_envelope = Some(envelope);
    }
}

impl ClientState for StateNoMoreRpc {
    fn name(&mut self) -> &'static str {
        "Exchanging CWMP RPCs"
    }

    fn on_enter(self: Box<Self>, session: &mut CWMPSession) -> Result<Box<dyn ClientState>> {
        Ok(Box::new(StateIdle))
    }
}

impl ClientState for StateExchangeRpc {
    fn name(&mut self) -> &'static str {
        "Exchanging CWMP RPCs"
    }

    fn on_enter(self: Box<Self>, session: &mut CWMPSession) -> Result<Box<dyn ClientState>> {
        if session.list_rpc.is_empty() {
            Ok(Box::new(StateNoMoreRpc))
        } else {
            Ok(self)
        }
    }
}

impl ClientState for StateIdle {
    fn name(&mut self) -> &'static str {
        "Idle"
    }

    fn on_enter(
        self: Box<Self>,
        session: &mut CWMPSession,
    ) -> crate::error::Result<Box<dyn ClientState>> {
        let msg_body = InformResponse { max_envelopes: 1 };
        let res = Envelope::new(
            &session.device_id.as_ref(),
            CWMPMsg::InformResponse(msg_body),
        );
        session.set_response_rpc(res);
        Ok(Box::new(StateExchangeRpc))
    }
}
