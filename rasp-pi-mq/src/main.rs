mod args;
mod heartbeat;
mod mqtt;
mod file;
mod errors;
mod ipc;
mod actions;

use clap::{AppSettings, App, ArgMatches};
use pretty_env_logger;
use std::env;
use log::{info,debug};
use uuid::Uuid;

use errors::{ResultExt, MyResult};

const APP_TITLE: &str = r#"
_____                 _____ _ __  __  ____  
|  __ \               |  __ (_)  \/  |/ __ \ 
| |__) |__ _ ___ _ __ | |__) || \  / | |  | |
|  _  // _` / __| '_ \|  ___/ | |\/| | |  | |
| | \ \ (_| \__ \ |_) | |   | | |  | | |__| |
|_|  \_\__,_|___/ .__/|_|   |_|_|  |_|\___\_\
                | |                          
                |_|                          
"#;

const APP_DESCRIPTION: &str = r#"
Rasp Pi

The Raspmberry MQ Interfacer
"#;

// Our CPNPN Messages
pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

// Main method to execute our MQTT Manager
// tag::basic[]
fn main() {
    env::set_var("RUST_LOG", env::var_os("RUST_LOG").unwrap_or_else(|| "info".into()));
    pretty_env_logger::init();
    info!("Starting Up MQTT Manager on Pi ...");
// end::basic[]
    let uuid = read_device_id().chain_err(|| "No device id file found").unwrap();
    let matches = command_line_args();    

    // tag::hb[]
    start_hearbeat(&matches, &uuid);
    // end::hb[]

    // Keep the heartbeat watching open
    loop {}
}


// tag::shb[]
fn start_hearbeat(matches: &ArgMatches, uuid: &Uuid,) {
    let server = matches.value_of(args::server::NAME).unwrap().to_string();
    let port = matches.value_of(args::port::NAME).unwrap().parse::<u16>().unwrap();

    let client_crt = matches.value_of(args::client_crt::NAME).unwrap().to_string();
    let client_key = matches.value_of(args::client_key::NAME).unwrap().to_string();
    let rootca = matches.value_of(args::rootca::NAME).unwrap().to_string();

    heartbeat::start(uuid.clone(), server, port, client_crt, client_key, rootca);
}
// end::shb[]

/**
 * This gets the device id for our device. We will set this for the device one time.
 * This will be created at build time. So its set once.
 */
// tag::device_id[]
 const UUID_LOCATION: &str = "/var/uuid";

 fn read_device_id() -> MyResult<Uuid> {
     use std::fs;
 
     let uuid_str = fs::read_to_string(UUID_LOCATION.to_string())?;  // <1>
 
     let uuid = Uuid::parse_str(uuid_str.trim()).unwrap();       // <2>
 
     debug!("Device UUID :: {:?}", uuid);
     Ok(uuid)
 }
// end::device_id[]

/**
 * This allows for arguments to be passed in by command line or environmental settings.
 */
// tag::cla[]
 fn command_line_args() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ColoredHelp)
        .arg(args::client_crt::declare_arg())
        .arg(args::client_key::declare_arg())
        .arg(args::rootca::declare_arg())
        .arg(args::server::declare_arg())
        .arg(args::port::declare_arg())        
        .get_matches()
}
// end::cla[]