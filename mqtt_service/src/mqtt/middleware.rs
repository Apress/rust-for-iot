
//use rumqtt::{ConnectionMethod, MqttClient, MqttOptions, QoS, ReconnectOptions, Notification};
use rumqtt::{MqttClient, Notification, Receiver};

use iron::{typemap, BeforeMiddleware, AfterMiddleware};
use iron::prelude::*;

use log::{debug};

// tag::middleware[]
use super::MqttClientConfig;
use super::client::create_client;

pub struct MqttClientMiddleware {
    config: MqttClientConfig,
    client: Option<MqttClient>
}

impl MqttClientMiddleware {
    pub fn new(config: MqttClientConfig) -> MqttClientMiddleware {
        MqttClientMiddleware {
            config: config,
            client: None
        }
    }
}
// end::middleware[]

// tag::create[]
// Our tuple struct
pub struct Value(MqttClientConfig, Option<MqttClient>); // <1>

// This part still confuses me a bit
impl typemap::Key for MqttClientMiddleware{ type Value = Value; }

impl BeforeMiddleware for MqttClientMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<MqttClientMiddleware>(Value(self.config.clone(), None));  // <2>
        Ok(())
    }
}

// TODO See if i can get working if not remove
//impl AfterMiddleware for MqttClientMiddleware {
//    fn after(&self, req: &mut Request,  res: Response) -> IronResult<Response> {
//        let config_val = req.extensions.get::<MqttClientMiddleware>().expect("Mqtt Client");
//        let Value(ref config, client) = config_val;
//        match client {
//            Some(mut client_val) => {
//                client_val.shutdown();
//            }
//            _ => {}
//        }
//        Ok(res)
//    }
//}


pub trait MqttRequestExt {
    fn mqtt_client(&mut self) -> (MqttClient, Receiver<Notification>);   // <3>
}

impl<'a, 'b> MqttRequestExt for Request<'a, 'b> {
    fn mqtt_client(&mut self) -> (MqttClient, Receiver<Notification>) {
        debug!("Get Client Request");
        let config_val = self.extensions.get::<MqttClientMiddleware>().expect("Mqtt Client");
        let Value(ref config, _) = config_val;

        // Create the client here for each request
        let (client, note) = create_client(&config, random().as_str());   // <4>
        // save the client
        self.extensions.insert::<MqttClientMiddleware>(Value(config.to_owned(), Some(client.clone())));

        return (client, note);
    }
}

fn random() -> String {         // <5>
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .collect()
}
// end::create[]