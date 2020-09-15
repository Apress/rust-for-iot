

use std::fs::File;
use std::io::prelude::*;
use log::{info,debug};
use crate::file::read;

// tag::create_client[]
use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions, Notification, Receiver};   // <1>

use super::MqttClientConfig;

const CLIENT_NAME: &str = "mqtt";

#[cfg(feature = "ch04")]
fn create_client(config: &MqttClientConfig, name: &str)
                      -> (MqttClient, Receiver<Notification>) {
    let client_id = format!("{}-{}-{}", CLIENT_NAME, name, config.server_name()); // <2>

    debug!("Create the connection ... ");

    let reconnection_options = ReconnectOptions::Always(10);

    let mqtt_options = MqttOptions::new(client_id, config.mqtt_server.as_str(), config.mqtt_port) // <3>
        .set_keep_alive(10)
        .set_reconnect_opts(reconnection_options)
        .set_clean_session(false);

    MqttClient::start(mqtt_options).expect("Issue trying to make a client connection")  // <4>
}
// end::create_client[]

#[cfg(feature = "full")]
// tag::client[]
pub fn create_client(config: &MqttClientConfig, name: &str)
    -> (MqttClient, Receiver<Notification>) {
    let ca = read(config.ca_crt.as_str());
    let server_crt = read(config.server_crt.as_str());
    let server_key = read(config.server_key.as_str());
    
    create_client_conn(config, ca, server_crt, server_key, name)
}
// end::client[]

fn create_client_conn(config: &MqttClientConfig,
                      ca: Vec<u8>,
                      server_crt: Vec<u8>,
                      server_key: Vec<u8>,
                      name: &str) -> (MqttClient, Receiver<Notification>) {
    let client_id = format!("{}-{}-{}", CLIENT_NAME, name, config.server_name()); // <2>

    debug!("Create the connection ... ");

    let reconnection_options = ReconnectOptions::Always(10);

    let mqtt_options = MqttOptions::new(client_id, config.mqtt_server.as_str(), config.mqtt_port) // <4>
        .set_keep_alive(10)
        .set_reconnect_opts(reconnection_options)
//        .set_ca(ca)
//        .set_client_auth(server_crt, server_key)
        .set_clean_session(false);

    MqttClient::start(mqtt_options).expect("Issue trying to make a client connection")  // <5>
}

///
/// Allows subscribing to an additional topic, the notifications created will report on it.
///
// tag::sub[]
pub fn subscribe(client: &mut MqttClient, topic: &'static str, qos: QoS) {
    info!("Subscribe and process: : {:?}", topic);
    client.subscribe(topic, qos).unwrap();
}
// end::sub[]

///
/// Publishing a payload.
///
// tag::pub[]
pub fn publish(client: &mut MqttClient, topic: &str, payload: String, qos: QoS) {
    info!("Publish to the topic : {:?}", topic);
    client.publish(topic, qos, false, payload).unwrap();
}
// end::pub[]