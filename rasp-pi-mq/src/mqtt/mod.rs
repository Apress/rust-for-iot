pub mod client;
pub mod processor;

// Copy trait needed for using this in treaded areas
#[derive(Debug, Clone)]
pub struct MqttClientConfig {
    pub ca_crt: String,
    pub server_crt: String,
    pub server_key: String,
    pub mqtt_server: String,
    pub mqtt_port: u16,
    pub uuid: String,
}

impl MqttClientConfig {
    pub fn server_name(&self) -> String {
        self.uuid.clone()
    }
}

use uuid::Uuid;

pub struct App<'a> {
    pub uuid: &'a Uuid,
    pub status: u16,
    pub msg: &'a str,
    pub peripherals: Vec<&'a str>
}

