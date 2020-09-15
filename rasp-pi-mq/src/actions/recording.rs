

use crate::mqtt::client::subscribe;
use crate::mqtt::processor::create_client;
use crate::mqtt::MqttClientConfig;
use super::monitor_notifications;
use rumqtt::QoS;

use uuid::Uuid;
use serde::{Serialize, Deserialize};

use log::{debug, info, error};

// tag::monitor[]
const MOD_NAME: &str = "recording";

pub fn monitor(config: &MqttClientConfig, device_id: &Uuid) {
    let (mut mqtt_client, notifications) = create_client(&config, MOD_NAME).unwrap(); // <1>
    info!("Subscribe to recording monitor ...");
    let topic = format!("recording/{:?}", device_id);                       // <2>
    subscribe(&mut mqtt_client, topic.as_str(), QoS::AtMostOnce);       // <3>
    debug!("Monitor the notifications ... ");
    monitor_notifications(notifications, process);                              // <4>
}
/**
 * Submit the recording to the other application
 */
pub fn process(topic: String, pl: Vec<u8>) {                    // <5>
    use serde_json::{Value};
    info!("Process Recording :: {}", topic);

    let pl = String::from_utf8(pl);         // <6>
    match pl {
        Ok(payload) => {
            let mut recording: Recording = serde_json::from_str(payload.as_str()).unwrap(); // <7>
            crate::ipc::send(recording.rtype)                                       // <8>
        },
        Err(error) => {
            error!("Error sending the payload: {:?}", error)
        },
    };
}
// end::monitor[]

// tag::struct[]
#[derive(Serialize, Deserialize, Debug)]
pub enum RecordingType {
    Start,
    Stop
}

#[derive(Serialize, Deserialize, Debug)]
struct Recording {
    uuid: Uuid,
    rtype: RecordingType
}
// end::struct[]