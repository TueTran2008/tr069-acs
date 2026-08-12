use crate::session::{cwmp_session::CWMPSession, device_actor::DeviceActor};
use dashmap::{mapref::one::Ref, DashMap};
use dioxus::html::input::list;
use kameo::actor::{ActorRef, Spawn};
use uuid::Uuid;

/*Contain */
pub struct DeviceSessionManager {
    list_devices: DashMap<String, ActorRef<DeviceActor>>,
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
        //let new_session = CWMPSession::new(device_indentity.to_string());
        let new_device = DeviceActor::spawn(DeviceActor::new(device_indentity.clone()));
        self.list_devices.insert(device_indentity, new_device);
    }

    pub fn get(&self, id: &String) -> Option<Ref<'_, String, ActorRef<DeviceActor>>> {
        self.list_devices.get(id)
    }
    //pub fn find_device(&self, device_id: &String) -> &CWMPSession {
    //    self.list_devices
    //        .iter()
    //        .find(|&s| s.get_device_id() == device_id)
    //        .unwrap()
    //}
}
