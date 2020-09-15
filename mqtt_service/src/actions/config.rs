
use iron::{Request, Response, IronResult};
use iron::status;
use log::{info};
use rumqtt::QoS;

use crate::mqtt::middleware::MqttRequestExt;
use crate::mqtt::client::publish;

use rumqtt::client::MqttClient;
use crate::mqtt::client::create_client;
use crate::mqtt::MqttClientConfig;

// @deprecated - we can use this later
pub fn update(req: &mut Request) -> IronResult<Response> {
    info!("Config Update");

    let (mut client, _ ) = req.mqtt_client();
    publish(&mut client, "config/123", "{\"name\": \"joseph\"}".to_string(), QoS::ExactlyOnce);

    Ok(Response::with((status::Ok, "OK")))
}