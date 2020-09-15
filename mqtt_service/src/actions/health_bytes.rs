
use log::{info,debug};
use rumqtt::QoS;
use uuid::Uuid;
use chrono::naive::NaiveDateTime;

use serde::{Serialize, Deserialize};

// For regex
use regex::Regex;
use lazy_static::lazy_static;

use crate::mqtt::client::{subscribe, create_client};
use crate::mqtt::{MqttClientConfig, monitor_notifications};

use crate::actions::health::HealthData;

const MOD_NAME: &str = "health_bytes";    // <1>

pub fn monitor(config: &MqttClientConfig, retrieval_svc_url: String) {
    let (mut mqtt_client, notifications) = create_client(&config, MOD_NAME); // <2>
    ////emqtt::subscribe(&config,"vehicle/+/pulse", &mandalore_url, heartbeat::process);
    info!("Subscribe to the device health bytes ...");
    subscribe(&mut mqtt_client,"health/bytes/+", QoS::AtMostOnce);    // <3>
    debug!("Monitor the notifications ... ");
    monitor_notifications(notifications, retrieval_svc_url,process);    // <4>
}

pub fn process(url: &str, topic: String, payload: Vec<u8>) {
    use serde_json::{Value};
    use crate::rpc::client;

    info!("Process Health :: {}", topic);

    client::run_health("127.0.0.1", 5555, payload);
}

fn convert_uuid_input(input: &str) -> Option<Uuid> {    // <3>
    match extract_uuid(input) {
        Some(uuid_str) => {
            let mut uuid = uuid_str;
            Some(Uuid::parse_str(uuid).unwrap())
        },
        None => {
            None
        }
    }
}

fn extract_uuid(input: &str) -> Option<&str> {
    debug!(" Input :: {}", input);
    lazy_static! {
        //static ref RE: Regex = Regex::new(r"health/(?P<uuid>[0-9a-zA-Z_-]*)/check").unwrap();
        static ref RE: Regex = Regex::new(r"health/bytes/(?P<id>(.*))").unwrap(); // <4>
    }

    RE.captures(input).and_then(|cap| {
        debug!("CAPTURE: {:?}", cap);
        cap.name("id").map(|uuid| uuid.as_str())    // <5>
    })
}

// end::process[]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        info!("Test Regex");
        assert_eq!(extract_uuid(r"health/123ABC"), Some(r"123ABC"));
        assert_eq!(extract_uuid(r"/health/ACDF-DE12"), Some(r"ACDF-DE12"));
    }
}
