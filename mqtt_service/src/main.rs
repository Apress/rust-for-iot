
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod mqtt;
mod file;
mod http;
mod actions;
mod parser;
mod rpc;

use pretty_env_logger;
use std::env;
use log::{info};

use crate::actions::health::monitor as health_monitor;
use crate::actions::health_bytes::monitor as health_bytes_monitor;


const APP_TITLE: &str = r#"
___  ________ _____ _____   _____  _   _ _____
|  \/  |  _  |_   _|_   _| /  ___|| | | /  __ \
| .  . | | | | | |   | |   \ `--. | | | | /  \/
| |\/| | | | | | |   | |    `--. \| | | | |
| |  | \ \/' / | |   | |   /\__/ /\ \_/ / \__/\
\_|  |_/\_/\_\ \_/   \_/   \____/  \___/ \____/
"#;

const APP_DESCRIPTION: &str = r#"
MQTT Service
Our MQTT Service that will interact with the message queue and forward the data back.
"#;

// For Argument matching
mod args;
use clap::{AppSettings, App, ArgMatches};

// tag::cap[]
pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}
// end::cap[]

fn main() {
    // Setup logger
    env::set_var("RUST_LOG", env::var_os("RUST_LOG").unwrap_or_else(|| "info".into()));
    pretty_env_logger::init();
    info!("Start MQTT  Server .. ");

    server();
}

/**
 * This allows for arguments to be passed in by command line or environmental settings.
 */
fn start_app_and_get_matches() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ColoredHelp)
        .arg(args::client_crt::declare_arg())
        .arg(args::client_key::declare_arg())
        .arg(args::rootca::declare_arg())
        .arg(args::port::declare_arg())
        .arg(args::server::declare_arg())
        .arg(args::http_port::declare_arg())
        .arg(args::http_server::declare_arg())
        .arg(args::rpc_server::declare_arg())
        .arg(args::rpc_port::declare_arg())
        .get_matches()
}

fn server() {
    let matches = start_app_and_get_matches();
    let server=matches.value_of(args::server::NAME).unwrap().to_string();
    let port=matches.value_of(args::port::NAME).unwrap().parse::<u16>().unwrap();

    let client_crt=matches.value_of(args::client_crt::NAME).unwrap().to_string();
    let client_key=matches.value_of(args::client_crt::NAME).unwrap().to_string();
    let rootca=matches.value_of(args::client_crt::NAME).unwrap().to_string();

    let http_server= matches.value_of(args::http_server::NAME).unwrap().to_string();
    let http_port= matches.value_of(args::http_port::NAME).unwrap().parse::<u16>().unwrap();

    let rpc_server= matches.value_of(args::rpc_server::NAME).unwrap().to_string();
    let rpc_port= matches.value_of(args::rpc_port::NAME).unwrap().parse::<u16>().unwrap();

    run_server(http_server, http_port,
               rpc_server, rpc_port,
               server, port, rootca, client_crt, client_key);
}

#[cfg(feature = "ch04")]
fn run_server(http_server: String, http_port: u16,
              rpc_server: String, rpc_port: u16,
              server: String, port: u16, rootca: String, client_crt: String, client_key: String) {
    use crate::mqtt::MqttClientConfig;

    info!("Start Secure Server");
    // create client
    let config = MqttClientConfig {
        mqtt_server: server,
        mqtt_port: port,
        ca_crt:  rootca,
        server_crt: client_crt,
        server_key: client_key,
        rpc_server: Some(rpc_server),
        rpc_port: Some(rpc_port)
    };

    health_monitor(&config, "http:://localhost:7001/".to_string());

    http::start(http_server.as_str(), http_port, config);
}

#[cfg(feature = "full")]
fn run_server(http_server: String, http_port: u16,
              rpc_server: String, rpc_port: u16,
              server: String, port: u16, rootca: String, client_crt: String, client_key: String) {
    use crate::mqtt::MqttClientConfig;

    info!("Start Secure Server");
    // create client
    let config = MqttClientConfig {
        mqtt_server: server,
        mqtt_port: port,
        ca_crt:  rootca,
        server_crt: client_crt,
        server_key: client_key,
        rpc_server: Some(rpc_server),
        rpc_port: Some(rpc_port)
    };

    health_bytes_monitor(&config, "http:://localhost:7001/".to_string());

    http::start(http_server.as_str(), http_port, config);
}
