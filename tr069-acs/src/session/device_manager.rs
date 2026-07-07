use crate::session::cwmp_session::CWMPSession;
use dashmap::DashMap;
use uuid::Uuid;

/*Contain */
pub struct DeviceSessionManager {
    list_devices: DashMap<String, CWMPSession>,
}

impl DeviceSessionManager {
    pub fn new() -> DeviceSessionManager {
        Self {
            list_devices: DashMap::new(),
        }
    }

    // A new session is added to the list when receiving Inform message
    pub fn insert_session(&mut self, device_indentity: String) {
        tracing::debug!("Insert new session wih device id {} ", device_indentity);
        let new_session = CWMPSession::new(device_indentity.to_string());
        self.list_devices.insert(device_indentity, new_session);
    }

    //pub fn find_device(&self, device_id: &String) -> &CWMPSession {
    //    self.list_devices
    //        .iter()
    //        .find(|&s| s.get_device_id() == device_id)
    //        .unwrap()
    //}
}
