use log::{info,debug};
use crate::file::read;
use super::MqttClientConfig;

use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions, Notification, Receiver};

use crate::errors::{ResultExt, MqttResult};

const CLIENT_NAME: &str = "mqtt";

pub fn create_client(config: &MqttClientConfig, name: &str) -> MqttResult<(MqttClient, Receiver<Notification>)> {
    let client_id = format!("{}-{}-{}", CLIENT_NAME.to_owned(), name, config.server_name());
    info!("Client connect :: {} / {}", config.mqtt_server, config.mqtt_port);
    debug!("File Read :: {} / {} / {}", config.ca_crt, config.server_crt, config.server_key);
    let ca = read(config.ca_crt.as_str());
    let server_crt = read(config.server_crt.as_str());
    let server_key = read(config.server_key.as_str());
    
    let reconnection_options = ReconnectOptions::Never;// Always(10);

    //let connection_method = ConnectionMethod::Tls(ca, Some((server_crt, server_key)));
    let mqtt_options = MqttOptions::new(client_id, config.mqtt_server.as_str(), config.mqtt_port)
        .set_keep_alive(10)
        .set_reconnect_opts(reconnection_options)
//        .set_ca(ca)
//        .set_client_auth(server_crt, server_key)
        .set_clean_session(false);
     
    MqttClient::start(mqtt_options)
}

///
/// Publishing a payload.
///
// tag::pub_bytes[]
pub fn publish_bytes(client: &mut MqttClient, topic: &str, payload: Vec<u8>, qos: QoS) {
    info!("Publish to the topic : {:?} / {:?}", topic, qos);
    client.publish(topic, qos, false, payload).unwrap();
}
// end::pub_bytes[]

pub fn publish(client: &mut MqttClient, topic: &str, payload: String, qos: QoS) {
    info!("Publish to the topic : {:?} / {:?}", topic, qos);
    client.publish(topic, qos, false, payload).unwrap();
}