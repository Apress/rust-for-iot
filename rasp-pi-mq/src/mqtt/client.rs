// MQTT Items
use crate::mqtt::MqttClientConfig;
use crate::mqtt::processor::{create_client, publish_bytes};
use super::App;
use rumqtt::{MqttClient, QoS};

use log::{info,debug,warn};

use std::time::{SystemTime,Duration};
use uuid::Uuid;

pub fn send(config: &MqttClientConfig, app: App) {
    use crate::errors::ErrorKind::MqttError;
    use crate::errors::{Error, ResultExt};
    info!("- Send Heartbeat -");

    let time_in_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    // create the u8 object
    let payload = build(app.status, app.msg, app.peripherals, &app.uuid, time_in_sec);

    match create_client(config, "OnClient") {
        Ok(res) => {
            info!("--- CLIENT MADE ---");
            let (mut client, _ ) = res;
            let topic = format!("health/bytes/{:?}", app.uuid);
            publish_bytes(&mut client, topic.as_str(), payload, rumqtt::QoS::AtLeastOnce);
        },
        Err(e) => {
            warn!("Error setting up client");
            let error = Error::from(MqttError(e));
            warn!("Unable to make a connection to the client :: {:?}", error);
        }
    }
}

// tag::capnp[]
fn build(status: u16, msg: &str, peripherals: Vec<&str>, uuid: &Uuid, time_in_sec: Duration) -> Vec<u8> {
    use crate::message_capnp::health;   // <1>
    use crate::message_capnp::health::Status;
    use capnp::serialize_packed;
    use capnp::traits::FromU16;     // <2>

    let mut message = ::capnp::message::Builder::new_default(); // <3>
    {
        // Use a Scope to limit lifetime of the borrow.
        let mut health = message.init_root::<health::Builder>();            // <4>
        // Get the user id (TODO hard code for now)
        health.set_user_id("JOSEPH1234");
        // Give it a unique ID
        health.set_uuid(uuid.to_string().as_ref());
        health.set_timestamp(time_in_sec.as_secs());
        // This could error if the result isnt 1-3, should validate or wrap
        health.set_status(Status::from_u16(status).unwrap());               // <5>
        health.set_msg(msg);

        // needs to occur after or you will get "value borrowed here after move" for hte other setters
        let mut health_peripherals = health.init_peripherals(peripherals.len() as u32);         // <6>
        {
            for i in 0..peripherals.len() {
                health_peripherals.reborrow().get(i as u32).set_name(peripherals[i]);           // <7>
            }
        }
    }

    // write the message to stdout
    let mut buffer = Vec::new();
    serialize_packed::write_message(&mut buffer, &message).unwrap();    // <8>

    debug!("Payload {:?} ", String::from_utf8(buffer.clone()));
    buffer
}
// end::capnp[]

pub fn subscribe<'a>(client: &'a mut MqttClient, topic: &str, qos: QoS) {
    info!("Subscribe and process: : {:?}", topic);
    client.subscribe(topic.clone(), qos).unwrap();
}