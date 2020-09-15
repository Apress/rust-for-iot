// tag::hb[]
use tokio::time::{interval_at, Duration, Instant};  // <1>

use uuid::Uuid;
use crate::mqtt::{App, MqttClientConfig};
use crate::mqtt::client::send as client_send; // <2>
use log::info;

use std::sync::Arc;

const INTERVAL_IN_SECONDS: u64 = 60 * 60;   // <3>

#[tokio::main]                              // <4>
pub async fn start(uuid: Uuid, server: String, port: u16,
               crt: String, key: String, ca: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setup and start our MQ ...");

    run(uuid, server, port, crt, key, ca);

    Ok(())
}

fn run( uuid: Uuid, server: String, port: u16,
            crt: String, key: String, ca: String) {
    let config = MqttClientConfig {     // <5>
        ca_crt:  ca,
        server_crt: crt,
        server_key: key,
        mqtt_server: server,
        mqtt_port: port,
        uuid: uuid.to_string()
    };
    let record_config = config.clone();

    let mut interval = interval_at(Instant::now(),  // <6>
                                   Duration::from_secs(INTERVAL_IN_SECONDS));   // <7>

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            send(&config, &uuid);   // <8>
        }
    });

    crate::actions::recording::monitor(&record_config, &uuid);
}

fn send(config: &MqttClientConfig, uuid: &Uuid) {   // <9>
    info!("Send Heartbeat for {}", uuid);
    let app = App { uuid: uuid,  status: 0, msg: "Everything is great ..", peripherals: vec!["Camera", "Sense HAT"]};
    client_send(config, app);
}
// end::hb[]