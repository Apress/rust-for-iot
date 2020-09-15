// to bring in the macros
// #![feature(custom_attribute)]
extern crate openssl;
#[macro_use]
extern crate diesel;
// Needed for the crates for the event sourcing derives
#[macro_use]
extern crate eventsourcing_derive;
extern crate eventsourcing;

use pretty_env_logger;
use std::env;
use log::{info};

// CQRS and GraphQL
mod graphql;
mod cqrs;
mod domain;

mod database;
mod models;
mod actions;
mod http;
mod errors;
mod rpc;
mod authorization;

// For Argument matching
mod args;
use clap::{AppSettings, App, ArgMatches, SubCommand};

const APP_TITLE: &str = r#"
___   ____ _____  ___   _   ____  _       __    _         __   _      __
| |_) | |_   | |  | |_) | | | |_  \ \  /  / /\  | |       ( (` \ \  / / /`
|_| \ |_|__  |_|  |_| \ |_| |_|__  \_\/  /_/--\ |_|__     _)_)  \_\/  \_\_,
"#;

const APP_DESCRIPTION: &str = r#"
Retrieval Services Application.
Application will store the meta data for a file and allow for query of the data.

The application can be interacted with multiple ways.

RPC
- the MQ service will call to it over RPC to send meta data for a movie and media

HTTP
- Http Port listeneres over GraphQL to Retireve ane Update Comments

EventSource
- EventSource listners are used by the CQRS app when storing an event. This is used for commentse which are triggerd from graphQL
"#;

pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

#[derive(Clone)]
struct Application {
    server: String,
    port: u16,
    rpc_port: u16,
    database: String,
    auth_server: String,
    event_store_host: String,
    event_store_port: u16,
    event_store_user: String,
    event_store_pass: String,
    event_store_web_port: u16,
}

fn main() {
    // Setup logger
    env::set_var("RUST_LOG", env::var_os("RUST_LOG").unwrap_or_else(|| "info".into()));
    pretty_env_logger::init();

    let matches: ArgMatches<'static> = start_app_and_get_matches();
    let app = Application {
        server: matches.value_of(args::server::NAME).unwrap().to_string(),
        port: matches.value_of(args::port::NAME).unwrap().parse::<u16>().unwrap(),
        rpc_port: matches.value_of(args::rpc::NAME).unwrap().parse::<u16>().unwrap(),
        database: matches.value_of(args::database::NAME).unwrap().to_string(),
        auth_server: matches.value_of(args::auth::NAME).unwrap().to_string(),
        event_store_host: matches.value_of(args::event_store_host::NAME).unwrap().to_string(),
        event_store_port: matches.value_of(args::event_store_port::NAME).unwrap().parse::<u16>().unwrap(),
        event_store_user: matches.value_of(args::event_store_user::NAME).unwrap().to_string(),
        event_store_pass: matches.value_of(args::event_store_pass::NAME).unwrap().to_string(),
        event_store_web_port: matches.value_of(args::event_store_web_port::NAME).unwrap().parse::<u16>().unwrap(),
    };

    // Decided if this is a migration or not
    // we only allow one or the other
    if let Some(_sub_m) = matches.subcommand_matches("migration") {
        subcommand_migrate(app.database.as_str());
    }
    else if let Some(_sub_m) = matches.subcommand_matches("rpc") {
        info!("Starting up RPC Server");
        rpc::server::start(app.server.as_str(), app.rpc_port, app.database.as_str()).unwrap();
    }
    else {
        run_http(app.clone());
    }
}


#[cfg(feature = "ch04")]
fn run_http(app: Application) {
    use std::thread;

    // Set variables
    let server = app.server;
    let port = app.port;
    let auth_server = app.auth_server;
    let database = app.database;

    let mut children = vec![];

    children.push(thread::spawn(move || {
        info!("Starting up application");
        http::start(server, port, auth_server, database);
    }));

    // Now join execute
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

// tag::cqrs[]
#[cfg(feature = "cqrs")]
fn run_http(app: Application) {
    use std::thread;

    let server = app.server;
    let port = app.port;
    let auth_server = app.auth_server;
    let database = app.database.clone();
    let es_host = app.event_store_host.clone();    // <1>
    let es_port = app.event_store_port;
    let es_user = app.event_store_user;
    let es_pass = app.event_store_pass;
    let es_web_port = app.event_store_web_port;

    // For cQRS call
    let es_host_cq = app.event_store_host;
    let es_port_cq = app.event_store_port;
    let db = app.database.clone();

    let mut children = vec![];

    children.push(thread::spawn(move || {           // <2>
        info!("Starting up application");
        http::start(server.as_str(), port, auth_server.as_str(), database.as_str(),
                    es_host.as_str(), es_web_port.clone(),
                    es_user, es_pass);
    }));

    children.push(thread::spawn(move || {       // <3>
        info!("Starting up CQRS");
        cqrs::start(db.as_str(),es_host_cq.as_str(), es_port_cq);
    }));

    // Now join execute
    for child in children {                     // <4>
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
// end::cqrs[]

/**
 * This allows for arguments to be passed in by command line or environmental settings.
 */
// tag::arg_matcher[]
fn start_app_and_get_matches() -> ArgMatches<'static> {
    App::new(APP_TITLE)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ColoredHelp)
        .arg(args::database::declare_arg())
        .arg(args::server::declare_arg())
        .arg(args::port::declare_arg())
        .arg(args::rpc::declare_arg())
        .arg(args::auth::declare_arg())
        .arg(args::event_store_host::declare_arg())
        .arg(args::event_store_port::declare_arg())
        .arg(args::event_store_pass::declare_arg())
        .arg(args::event_store_user::declare_arg())
        .arg(args::event_store_web_port::declare_arg())
        .subcommand(SubCommand::with_name("migration")
            .about("runs the migrations for diesel"))
        .subcommand(SubCommand::with_name("rpc")
            .about("runs the RPC server"))
        .get_matches()
}
// end::arg_matcher[]

use crate::errors::{ResultExt, MyResult};

// tag::dieselm[]
#[macro_use] extern crate diesel_migrations;
embed_migrations!("./migrations");      // <1>

fn subcommand_migrate(database_url: &str) -> MyResult<()> {
    use diesel::pg::PgConnection;
    use diesel::Connection;

    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    info!("Running migrations");

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()) // <2>
        .chain_err(|| "Error running migrations")?;

    info!("Finished migrations");

    Ok(())
}
// end::dieselm[]