use config::{Config, ConfigError};
use serde::Deserialize;
use std::sync::OnceLock;

pub fn core_config() -> &'static GlobalConfigSetting {
    static INSTANCE: OnceLock<GlobalConfigSetting> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        GlobalConfigSetting::get_setting()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfigSetting {
    pub database: DatabaseSetting,
    //pub mqtt: MqttSetting,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/*
*   The SRV URI connection scheme has the following form:
*       mongodb+srv://[username:password@]host[/[defaultauthdb][?options]]
*   Please check the https://www.mongodb.com/docs/manual/reference/connection-string/ to see more
*   document about connection string
*
* */

impl GlobalConfigSetting {
    //mongodb:.//user:pass@sample.host:port/?Optio

    pub fn get_database_connect_string(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name
        )
    }

    pub fn get_setting() -> Result<Self, ConfigError> {
        let setting = Config::builder()
            .add_source(config::File::with_name("configuration.yaml"))
            .build()
            .unwrap();
        setting.try_deserialize()
    }
}
