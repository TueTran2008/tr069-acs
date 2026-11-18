use crate::{model::ModelManager, session::device_actor::DeviceActor};
use dashmap::DashMap;
use kameo::actor::{ActorRef, Spawn};

/*Contain */
pub struct DeviceSessionManager {
    list_devices: DashMap<String, ActorRef<DeviceActor>>,
    mm: ModelManager,
}

impl DeviceSessionManager {
    pub async fn new() -> Self {
        let mm = ModelManager::new().await.unwrap();
        Self {
            list_devices: DashMap::new(),
            mm,
        }
    }

    // A new session is added to the list when receiving Inform message
    pub fn insert_session(&mut self, device_indentity: String) {
        tracing::debug!("Insert new session wih device id {} ", device_indentity);
        let new_device =
            DeviceActor::spawn(DeviceActor::new(device_indentity.clone(), self.mm.clone()));
        self.list_devices.insert(device_indentity, new_device);
    }

    pub fn get(&self, id: &String) -> Option<ActorRef<DeviceActor>> {
        self.list_devices.get(id).map(|entry| entry.value().clone())
    }
}
