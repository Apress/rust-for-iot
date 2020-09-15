
// for the web services
use iron::{Iron, Request, Response, IronResult, status};
use iron::prelude::Chain;
use router::{Router};
use router::router;
use mount::Mount;

use crate::actions::{config,recording};
use crate::mqtt::MqttClientConfig;
use crate::mqtt::middleware::MqttClientMiddleware;

use log::info;

pub fn start(server: &str, port: u16, config: MqttClientConfig) {
    let mut mount = Mount::new();

    let router = create_routes();
    mount.mount("/api", router);

    let mut chain = Chain::new(mount);
    create_mqtt_handler(&mut chain, config);

    // start up the service
    info!("Start Server on {}:{}", server, port);
    Iron::new(chain).http(format!("{}:{}", server, port));
}

fn create_mqtt_handler(chain: &mut Chain, config: MqttClientConfig) {
    let middleware = MqttClientMiddleware::new(config);

    chain.link_before(middleware);
}

/**
 * Configs can go both ways.
 */
fn create_routes() -> Router {
    router!(
        health: get "/healthz" => health,
        update_config: put "/config/:id/update" => config::update,
        // tag::router[]
        recording: post "/recording/:id/:type" => recording::run,
        // end::router[]
    )
}

pub fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}
