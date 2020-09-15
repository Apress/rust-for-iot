
use iron::{Request, Response, IronResult};
use iron::status;
use log::{info};

use uuid::Uuid;
use serde::{Serialize, Deserialize};

// tag::run[]
use crate::mqtt::middleware::MqttRequestExt;
use crate::mqtt::client::publish;
use rumqtt::QoS;

pub fn run(req: &mut Request) -> IronResult<Response> {
    info!("Recording Start/Stop");

    let recording = Recording {                 // <1>
        uuid: super::get_uuid_value(req, "id"),
        rtype: get_recording_type(req)
    };
    info!("Set Recording Type to : {:?}", recording);

    // Send the data over
    let (mut client, _ ) = req.mqtt_client();   // <2>
    let topic = format!("recording/{}", recording.uuid);
    let json = serde_json::to_string(&recording).unwrap(); // <3>

    publish(&mut client, topic.as_str(), json, QoS::AtLeastOnce); // <4>

    Ok(Response::with((status::Ok, "OK")))
}

fn get_recording_type(req: &Request) -> RecordingType {
    match super::get_value(req, "type").as_ref() {
        "start" => RecordingType::Start,
        "stop" => RecordingType::Stop,
        _ => panic!("You have bad code")
    }
}

// end::run[]

// tag::struct[]
#[derive(Serialize, Deserialize, Debug)]
enum RecordingType {    // <1>
    Start,
    Stop
}


#[derive(Serialize, Deserialize, Debug)]
struct Recording {  // <2>
    uuid: Uuid,
    rtype: RecordingType
}
// end::struct[]