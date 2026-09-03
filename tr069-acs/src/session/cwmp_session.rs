use tracing_log::log;

//pub struct StateIdle;
//
use crate::error::Result;
use crate::{
    cwmp_msg::{CWMPMsg, Envelope},
    session::state::client_state::ClientState,
    soap::inform::InformResponse,
};

pub struct StateIdle; // When recevied Inform message from CWMP Client
pub struct StateExchangeRpc;
pub struct StateNoMoreRpc;

pub enum CWMPStateAction {
    CWMPStateReceiveInform(String),
    CWMPStateReceiveResponse,
    CWMPStateReceiveEmpty,
    CWMPStateTransition(Box<dyn ClientState>), // Pass in next state
    CWMPStateEnd,
}
//#[derive(Eq, Hash, PartialEq)]
pub struct CWMPSession {
    response_envelope: Option<Envelope>,
    msg_id: Option<String>,
    device_id: String,
    list_rpc: Vec<CWMPMsg>,
    state: Box<dyn ClientState>,
}

impl CWMPSession {
    pub fn new(device_id: String) -> Self {
        Self {
            response_envelope: None,
            msg_id: None,
            device_id,
            list_rpc: Vec::new(),
            state: Box::new(StateIdle),
        }
    }

    pub async fn apply_action(&mut self, action: CWMPStateAction) -> Result<Option<Envelope>> {
        match action {
            /*
             * Typical ACS processing:
             * Authenticate device
             * Parse DeviceId
             * Update Last Inform Time
             * Update IP
             * Upddate Software Version
             * Upddate Serial Number
             * Saved Event list
             * Detdermine why the device connected
             * */
            CWMPStateAction::CWMPStateReceiveInform(msg_id) => {
                self.msg_id = Some(msg_id);
                let msg_body = InformResponse { max_envelopes: 1 };
                let res = Envelope::new(
                    self.msg_id.as_ref().unwrap(),
                    CWMPMsg::InformResponse(msg_body),
                );
                self.transition_to(Box::new(StateExchangeRpc));
                return Ok(Some(res));
            }
            /*CPE sends Empty HTTP POST ~ "CPE is asking do you have any command for me" */
            CWMPStateAction::CWMPStateReceiveEmpty => {
                if let Some(next_rpc) = self.list_rpc.pop() {
                    let res = Envelope::new(self.msg_id.as_ref().unwrap(), next_rpc);
                    return Ok(Some(res));
                } else {
                    self.transition_to(Box::new(StateNoMoreRpc));
                    return Ok(None);
                }
            }
            /*Do action base on the the RPC response from Device*/
            CWMPStateAction::CWMPStateReceiveResponse => {
                return Ok(None);
            }
            CWMPStateAction::CWMPStateTransition(next) => {
                self.transition_to(next);
                return Ok(None);
            }
            _ => todo!("implement other"),
        };
        //Ok(None)
    }

    fn transition_to(&mut self, next_state: Box<dyn ClientState>) -> Result<()> {
        let next_state_name = next_state.name();
        log::debug!(
            "Client {} transit to state:{}",
            self.get_device_id(),
            next_state_name
        );
        self.state = next_state;
        //self.state.on_enter(self)
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
    fn name(&self) -> &'static str {
        "Exchanging CWMP RPCs"
    }

    fn on_enter(self: Box<Self>, session: CWMPStateAction) -> Result<Box<dyn ClientState>> {
        Ok(Box::new(StateIdle))
    }
}

impl ClientState for StateExchangeRpc {
    fn name(&self) -> &'static str {
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
    fn name(&self) -> &'static str {
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
