
use pretty_env_logger;
use std::env;
use log::{info};

mod http;
mod actions;
mod errors;
mod parsers;

// For Argument matching
mod args;
use clap::{AppSettings, App, ArgMatches};

const APP_TITLE: &str = r#"
_     ___   _     ___    __    ___       __   _      __
| | | | |_) | |   / / \  / /\  | | \     ( (` \ \  / / /`
\_\_/ |_|   |_|__ \_\_/ /_/--\ |_|_/     _)_)  \_\/  \_\_,
"#;

const APP_DESCRIPTION: &str = r#"
The Upload and Download Service deals with uplaoding and downloading the application video and image files.
This will then on upload send over the meta data to retrieval service for storage.
"#;

fn main() {
    // Setup logger
    env::set_var("RUST_LOG", env::var_os("RUST_LOG").unwrap_or_else(|| "info".into()));
    pretty_env_logger::init();

    let matches = get_matches();
    let server = matches.value_of(args::server::NAME).unwrap();
    let port = matches.value_of(args::port::NAME).unwrap().parse::<u16>().unwrap();
    let retrieval_svc_url = matches.value_of(args::retrieval_url::NAME).unwrap();

    info!("-- Start Upload Service --");
    // needs 0.0.0.0 for container deployments
    http::start(server, port, retrieval_svc_url);
}


/**
 * This allows for arguments to be passed in by command line or environmental settings.
 */
fn get_matches() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ColoredHelp)
        .arg(args::server::declare_arg())
        .arg(args::port::declare_arg())
        .arg(args::retrieval_url::declare_arg())
        .get_matches()
}