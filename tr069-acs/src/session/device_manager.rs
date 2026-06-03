use crate::session::cwmp_session::CWMPSession;

pub struct DeviceSessionManager {
    list_devices: Vec<CWMPSession>,
}

impl DeviceSessionManager {
    pub fn new() -> DeviceSessionManager {
        Self {
            list_devices: Vec::new(),
        }
    }

    // A new session is added to the list when receiving Inform message
    pub fn insert_session(&mut self, device_id: String) {
        tracing::debug!("Insert new session wih device id {} ", device_id);
        let new_session = CWMPSession::new(device_id);
        self.list_devices.push(new_session);
    }

    pub fn find_device(&self, device_id: &String) -> &CWMPSession {
        self.list_devices
            .iter()
            .find(|&s| s.get_device_id() == device_id)
            .unwrap()
    }
}
