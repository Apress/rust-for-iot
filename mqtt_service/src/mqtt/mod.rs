
pub mod client;
pub mod middleware;

// tag::struct[]
#[cfg(feature = "ch04")]
#[derive(Debug, Clone)]
pub struct MqttClientConfig {
    pub mqtt_server: String,    // <1>
    pub mqtt_port: u16,         // <2>
}

#[cfg(feature = "ch04")]
impl MqttClientConfig {
    pub fn server_name(&self) -> String {   // <3>
        match sys_info::hostname() {
            Ok(name) => {
                name
            },
            Err(error) => {
                "server".to_string()
            }
        }
    }
}
// end::struct[]

#[cfg(feature = "full")]
// tag::config[]
#[derive(Debug, Clone)]
pub struct MqttClientConfig {
    pub ca_crt:  String,
    pub server_crt: String,
    pub server_key: String,
    pub mqtt_server: String,
    pub mqtt_port: u16,
    // for the RPC
    pub rpc_server: Option<String>,
    pub rpc_port: Option<u16>,
}
// end::config[]

#[cfg(feature = "full")]
impl MqttClientConfig {
    pub fn server_name(&self) -> String {
        match sys_info::hostname() {
            Ok(name) => {
                name
            },
            Err(error) => {
                "server".to_string()
            }
        }
    }
}

// Chapter 4
// tag::monitor[]
use rumqtt::{Receiver, Notification}; // <1>
use rumqtt::Notification::{Publish};
use std::{thread, str};
use log::{debug, error};

// Monitor the notifications from the notification receiver.
#[cfg(feature = "ch04")]
pub fn monitor_notifications(notifications: Receiver<Notification>, url: String, f: fn(&str, String, String)) { // <2>
    thread::spawn(move || { // <3>
        for notification in notifications {
            debug!("Notification {:?}", notification);
            match notification {
                Publish(p) => { // <4>
                    // Retrieve the payload and convert it to a string.
                    let pl = String::from_utf8(p.payload.to_vec());
                    debug!("The Payload :: PKID: {:?}, QOS: {:?}, Duplicate: {:?}", p.pkid, p.qos, p.dup);
                    match pl {
                        Ok(payload) => f(url.as_str(), p.topic_name, payload),  // <5>
                        Err(error) => {
                            error!("Error sending the payload: {:?}", error)
                        },
                    };
                },
                _ => debug!("n/a")
            }
        }
    });
}
// end::monitor[]

// Full - Cap'n Proto
// Monitor the notifications from the notification receiver.
#[cfg(feature = "full")]
pub fn monitor_notifications(notifications: Receiver<Notification>, url: String, f: fn(&str, String, Vec<u8>)) { // <2>
    thread::spawn(move || { // <3>
        for notification in notifications {
            debug!("Notification {:?}", notification);
            match notification {
                Publish(p) => { // <4>
                    // Retrieve the payload and convert it to a string.
                    let pl = p.payload.to_vec();
                    debug!("The Payload :: PKID: {:?}, QOS: {:?}, Duplicate: {:?}", p.pkid, p.qos, p.dup);
                    f(url.as_str(), p.topic_name, pl);  // <5>
                },
                _ => debug!("n/a")
            }
        }
    });
}
