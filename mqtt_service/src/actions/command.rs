
use iron::{Request, Response, IronResult};
use iron::status;
use log::{info};
use rumqtt::QoS;

use crate::mqtt::middleware::MqttRequestExt;
use crate::mqtt::client::publish;

use rumqtt::client::MqttClient;
use crate::mqtt::client::create_client;
use crate::mqtt::MqttClientConfig;

// Setup to send any command through
// Right now sends a caputre image
// and display temperature
pub fn run(req: &mut Request) -> IronResult<Response> {
    info!("Send Command");

    // Send the data over
    let (mut client, _ ) = req.mqtt_client();
    let topic = format!("command/{}", recording.uuid);
    let cmd = Command::new(&req);
    let json = serde_json::to_string(&recording).unwrap();

    publish(&mut client, topic.as_str(), json, QoS::AtLeastOnce);

    Ok(Response::with((status::Ok, "OK")))
}

impl Command {
    fn new(req: &Request) -> Command {
        let command = match command {
            "image" => CommandTypes::CaptureImage,
            "temp" => CommandTypes::ShowTemp,
            _ => panic!("Bad parameter")
        };
        Command {
            uuid: super::get_uuid_value(req, "id"),
            command: command,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum CommandTypes {
    CaptureImage,
    ShowTemp
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    uuid: Uuid,
    command: CommandTypes,
}
