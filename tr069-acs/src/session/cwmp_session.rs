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

pub enum CWMPStateAction {
    CWMPStateReceiveInform,
    CWMPStateReceiveResponse,
    CWMPStateTransition(Box<dyn ClientState>), // Pass in next state
    CWMPStateEnd,
}
//#[derive(Debug)]
pub struct CWMPSession {
    response_envelope: Option<Envelope>,
    device_id: String,
    list_rpc: Vec<CWMPMsg>,
    state: Box<dyn ClientState>,
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

    pub fn apply_action(&self, action: CWMPStateAction) -> Result<()> {
        match action {
            CWMPStateAction::CWMPStateTransition()
        }
        Ok(())
    }
    pub fn transition(&mut self, next_state: CWMPStateAction) -> Result<()> {
        Ok(())
    }

    pub fn set_response_rpc(&mut self, envelope: Envelope) {
        self.response_envelope = Some(envelope);
    }

    pub fn get_device_id(&self) -> &String {
        &self.device_id
    }

    pub fn transition(&mut self) {
        //self.state = self.state.on_enter();
        //self.state.on_enter(session)
        //let state = std::mem::replace(&mut self.state, Box::new(StateIdle));
        //let new_state = state.on_enter(self).unwrap();
        //self.state = new_state;
    }
}

impl ClientState for StateNoMoreRpc {
    fn name(&mut self) -> &'static str {
        "Exchanging CWMP RPCs"
    }

    fn on_enter(self: Box<Self>, session: CWMPStateAction) -> Result<Box<dyn ClientState>> {
        Ok(Box::new(StateIdle))
    }
}

impl ClientState for StateExchangeRpc {
    fn name(&mut self) -> &'static str {
        "Exchanging CWMP RPCs"
    }

    fn on_enter(self: Box<Self>, session: CWMPStateAction) -> Result<Box<dyn ClientState>> {
        //if session.list_rpc.is_empty() {
        Ok(Box::new(StateNoMoreRpc))
        //} else {
        //    Ok(self)
        //}
    }
}

impl ClientState for StateIdle {
    fn name(&mut self) -> &'static str {
        "Idle"
    }

    fn on_enter(
        self: Box<Self>,
        session: CWMPStateAction,
    ) -> crate::error::Result<Box<dyn ClientState>> {
        //let msg_body = InformResponse { max_envelopes: 1 };
        //let res = Envelope::new(
        //    &session.device_id.as_ref(),
        //    CWMPMsg::InformResponse(msg_body),
        //);
        //session.set_response_rpc(res);
        Ok(Box::new(StateExchangeRpc))
    }
}
