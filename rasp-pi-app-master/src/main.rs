
//use tokio::prelude::*;
use clap::{AppSettings, App, ArgMatches};

use pretty_env_logger;
use std::env;
use log::{info,debug, error};

use errors::{ResultExt, MyResult};

mod args;
mod file;
mod errors;
mod manager;

mod led;
mod sensors;
mod daily;
mod joystick;
mod camera;
mod homekit;

use led::screen::LedControls;
use sensors::atmospheric::Atmospheric;

use manager::{Tx};
use uuid::Uuid;

const APP_NAME: &str = "Rasp Pi";

const APP_TITLE: &str = r#"
______               ______ _   _____ _ _            _
| ___ \              | ___ (_) /  __ \ (_)          | |
| |_/ /__ _ ___ _ __ | |_/ /_  | /  \/ |_  ___ _ __ | |_
|    // _` / __| '_ \|  __/| | | |   | | |/ _ \ '_ \| __|
| |\ \ (_| \__ \ |_) | |   | | | \__/\ | |  __/ | | | |_
\_| \_\__,_|___/ .__/\_|   |_|  \____/_|_|\___|_| |_|\__|
| |
|_|
"#;

const APP_DESCRIPTION: &str = r#"
Rasp Pi

The Raspmberry Pi Client Application
"#;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Application {
    backend: String,
    uuid: String
}

fn main() {
    // Setup logger
    env::set_var("RUST_LOG", env::var_os("RUST_LOG").unwrap_or_else(|| "info".into()));
    pretty_env_logger::init();
    info!("Starting Up Device ...");

    let matches = command_line_args();

    let uuid = read_device_id().chain_err(|| "No device id file found").unwrap().to_string();

    // Authenticate the user, first thing we are going to do when entering
    // the application
    run_authentication(&matches);

    // Start our background processes
    //uuid.to_hyphenated().to_string()
    let app = Application {
        backend: matches.value_of(args::backend::NAME).unwrap().to_string(),
        uuid,
    };

    run(app.clone());
}

// https://rust-lang.github.io/async-book/03_async_await/01_chapter.html#async-move
//`threaded_scheduler` - Uses the multi-threaded scheduler. Used by default.
// Arc - for the asyncrhsonus threads
// Mutex - for mutual exclusion locking
#[cfg(feature = "full")]
#[tokio::main]
//async fn run(backend: &'static str, uuid: &'static str) -> Result<(), Box<dyn std::error::Error>> {
async fn run(app: Application) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::sync::{mpsc, oneshot, watch};

    info!("Setup and start our channel runners - full ...");

    // for multi producer, multi consumer
    let (tx, rx) = mpsc::channel(100);
    let joy_tx: Tx = tx.clone();
    let daily_tx: Tx = tx.clone();
    let temp_cmd_tx: Tx = tx.clone();
    // tag::face[]
    let motion_cmd_tx: Tx = tx.clone();

    // basically like a watcher but we needed features that watcher didnt provide
    // Single producer/ consumer for this setup
    let (face_tx, face_rx) = mpsc::channel(1);

    // Face detector, for single producer single consumer
    // One shot can not be used in loops its deisgned for one shot
    // let (motion_tx, motion_rx) = oneshot::channel::<bool>();
    let (motion_tx, motion_rx) = watch::channel(false);
    // end::face[]

    // Temp Detection, for single producer, multi consumer
    // channel sets the initial value, we will set to room temperature
    let (temp_tx, mut temp_rx) = watch::channel(25f32);

    // Start our timer matcher
    // we want to do this after the authentication so we don't have any interruption from the
    // login; this will also run Asynchronously
    daily::run(daily_tx);

    // Setup and run the Joystick now;
    joystick::run(joy_tx);

    // Captures the video camera
    camera::run_video_capture(face_tx);

    // send the camera recording on an hourly basis
    camera::hourly_upload(app.uuid, app.backend);

    // TODO : Send temperature to file
    // Run our home kit setup
    homekit::initialize(motion_cmd_tx, motion_rx, temp_cmd_tx, temp_rx);

    // Ready our receivers
    let led_controls = Arc::new(Mutex::new(LedControls::new()));
    let atmospheric = Arc::new(Mutex::new(Atmospheric::new()));

    manager::run(rx, temp_tx, face_rx, motion_tx, &led_controls, &atmospheric).await;

    debug!("Complete");
    Ok(())
}

#[cfg(feature = "ch09")]
// tag::tokio[]
#[tokio::main]      // <1>
async fn run(matches: &ArgMatches, uuid: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::sync::mpsc;

    info!("Setup and start our channel runners ...");

    // defines the buffer to send in
    let (tx, rx) = mpsc::channel(100);  // <2>
    let joy_tx: Tx = tx.clone();        // <3>
    let daily_tx: Tx = tx.clone();

    // Start our timer matcher
    // we want to do this after the authentication so we don't have any interruption from the
    // login; this will also run Asynchronously
    daily::run(daily_tx);                               // <4>

    // Setup and run the Joystick now;
    joystick::run(joy_tx);                              // <5>

    // Ready our receivers
    let led_controls = Arc::new(Mutex::new(LedControls::new()));    // <6>
    let atmospheric = Arc::new(Mutex::new(Atmospheric::new()));

    manager::run(rx, &led_controls, &atmospheric).await;            // <7>

    debug!("Complete");
    Ok(())
}
// end::tokio[]

// tag::auth[]
#[tokio::main]
async fn run_authentication(matches: &ArgMatches) {
    use authentication::Access;

    info!("Run Authentication ...");

    // Initialize the LED controls
    let led_controls = Arc::new(Mutex::new(LedControls::new()));


    let client_id = matches.value_of(args::auth_client_id::NAME).unwrap().to_string();
    let client_secret = matches.value_of(args::auth_client_secret::NAME).unwrap().to_string();
    let url = matches.value_of(args::auth0::NAME).unwrap().to_string();

    let access = Access::new(client_id, client_secret, url, led_controls);

    // Authenticate
    if access.authenticate().await == false {
        error!("Not Logged In:");
    }
}
// end::auth[]

#[derive(Clone)]
struct LedVisualDisplay<'a> {
    led: &'a Arc<Mutex<LedControls>>
}

// tag::visual[]
impl authentication::VisualDisplay for LedControls {
    fn clear(&mut self) {
        // let mut led_control_unwrap = self.led.lock().unwrap();
        // led_control_unwrap.blank();
        self.blank();
    }

    fn display_text(&mut self, text: &str) {
        // let mut led_control_unwrap = self.led.lock().unwrap();        
        // led_control_unwrap.scroll_text(text);
        self.scroll_text(text);
    }

    fn display_processing(&mut self) {
        // let mut led_control_unwrap = self.led.lock().unwrap();
        // led_control_unwrap.processing();
        self.processing();
    }
}
// end::visual[]

/**
 * This allows for arguments to be passed in by command line or environmental settings.
 */
fn command_line_args() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ColoredHelp)
        .arg(args::auth0::declare_arg())
        .arg(args::auth_client_id::declare_arg())
        .arg(args::auth_client_secret::declare_arg())
        .arg(args::backend::declare_arg())
        .get_matches()
}

const UUID_LOCATION: &str = "/var/uuid";

fn read_device_id() -> MyResult<Uuid> {
    use std::fs;

    let uuid_str = fs::read_to_string(UUID_LOCATION.to_string())?;  // <1>

    let uuid = Uuid::parse_str(uuid_str.trim()).unwrap();       // <2>

    debug!("Device UUID :: {:?}", uuid);
    Ok(uuid)
}

// cargo build --target=armv7-unknown-linux-musleabihf
