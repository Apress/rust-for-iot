
use log::{info,debug,error};
use rumqtt::QoS;
use uuid::Uuid;
use chrono::naive::NaiveDateTime;

use serde::{Serialize, Deserialize};

// For regex
use regex::Regex;
use lazy_static::lazy_static;

// tag::monitor[]
use crate::mqtt::client::{subscribe, create_client};
use crate::mqtt::{MqttClientConfig, monitor_notifications};

const MOD_NAME: &str = "health";    // <1>

pub fn monitor(config: &MqttClientConfig, retrieval_svc_url: String) {
    let (mut mqtt_client, notifications) = create_client(&config, MOD_NAME); // <2>
    ////emqtt::subscribe(&config,"vehicle/+/pulse", &mandalore_url, heartbeat::process);
    info!("Subscribe to the device health ...");
    subscribe(&mut mqtt_client,"health/+", QoS::AtMostOnce);    // <3>
    debug!("Monitor the notifications ... ");
    monitor_notifications(notifications, retrieval_svc_url,process);    // <4>
}
// end::monitor[]

// tag::process[]
pub fn process(url: &str, topic: String, pl: Vec<u8>) {
    info!("Payload Size :: {} ", pl.len());
    let pl = String::from_utf8(pl);
    match pl {
        Ok(payload) => process_and_send(url, topic, payload),  // <5>
        Err(error) => {
            error!("Error sending the payload: {:?}", error)
        },
    };
}

fn process_and_send(url: &str, topic: String, payload: String) {
    info!("Process Health :: {} :: {}", topic, payload);

    // UUID and data
    let uuid = convert_uuid_input(topic.as_ref());  // <1>

    let mut health_data: HealthData = serde_json::from_str(payload.as_str()).unwrap(); // <2>
    health_data.uuid = uuid;

    send_to_service(url, &health_data);
}

fn convert_uuid_input(input: &str) -> Option<Uuid> {    // <3>
    match extract_uuid(input) {
        Some(uuid_str) => {
            let uuid = uuid_str;
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
        static ref RE: Regex = Regex::new(r"health/(?P<id>(.*))").unwrap(); // <4>
    }

    RE.captures(input).and_then(|cap| {
        debug!("CAPTURE: {:?}", cap);
        cap.name("id").map(|uuid| uuid.as_str())    // <5>
    })
}

pub fn send_to_service(url: &str, data: &HealthData){
    info!("We are sending {:?} to {}", data, url);  // <6>
}
// end::process[]

// tag::struc[]
use chrono::prelude::*;
use chrono::naive::serde::ts_milliseconds::deserialize as from_milli_ts;    // <1>

// combination of :  use num_traits::{FromPrimitive,ToPrimitive};
use enum_primitive_derive::Primitive;

#[derive(Serialize, Deserialize, Debug, Primitive, PartialEq)]
pub enum Status {   // <2>
    Green = 0,
    Red = 1,
    Yellow = 2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripheral {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthData {
    pub uuid: Option<Uuid>,
    #[serde(deserialize_with = "from_milli_ts")]    // <3>
    pub timestamp: NaiveDateTime,
    pub status: Status,
    pub msg: String,
    pub peripherals: Vec<Peripheral>
}
// end::struc[]

// Created for chapter 5
// tag::create_new_health[]
use chrono::{DateTime,Utc};
use num_traits::cast::FromPrimitive;

impl HealthData {
    pub fn new(uuid: &str, timestamp: u64, status: u16, msg: &str, ps: Vec<Peripheral>) -> HealthData {
        HealthData {
            uuid: Some(Uuid::parse_str(uuid).unwrap()),
            timestamp: HealthData::convert_time(timestamp),
            status: Status::from_u16(status).unwrap(),
            msg: msg.to_string(),
            peripherals: ps
        }
    }

    fn convert_time(millis: u64) -> NaiveDateTime {
        use chrono::NaiveDateTime;

        let seconds = (millis / 1000) as i64;
        let nanos = ((millis % 1000) * 1_000_000) as u32;
        //DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
        NaiveDateTime::from_timestamp(seconds, nanos)
    }
}
// end::create_new_health[]

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
