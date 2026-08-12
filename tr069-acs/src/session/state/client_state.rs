//This will simulate the CWMP client state from begining of the session to the end of the session

use crate::error::Result;
use crate::session::cwmp_session::CWMPStateAction;

pub trait ClientState: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_enter(self: Box<Self>, session: CWMPStateAction) -> Result<Box<dyn ClientState>>;
}
