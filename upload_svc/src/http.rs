
// for the web services
use iron::{Iron, Request, Response, IronResult, status};
use iron::prelude::Chain;
use mount::Mount;

// For the multipart

use log::{info};

use crate::actions::download::download;
use crate::actions::upload::upload;

pub fn start(server: &str, port: u16, retrieval_svc_url: &str) {
    // mounts the router to the
    let mut mount = Mount::new();

    // Api Routes we will setup and support
    let router = create_routes(retrieval_svc_url);
    mount.mount("/api", router);

    let mut chain = Chain::new(mount);
    create_links(&mut chain);

    // create the handler and bind it it to a port
    info!("Start Server on {}:{}", server, port);
    Iron::new(chain).http(format!("{}:{}", server, port));
}

use std::path::PathBuf;

fn create_links(chain: &mut Chain) {
    use multipart::server::iron::{Intercept, LimitBehavior};

    // link the chain
    // in crease some of the file size and size limits
    // these are some large file sizes
    let intercept = Intercept {
        temp_dir_path: None,
        file_size_limit: 2 * 1024 * 1024 * 1000,
        file_count_limit: 16,
        limit_behavior: LimitBehavior::ThrowError,
    };

    chain.link_before(intercept);
}

// tag::routes[]
use router::{Router};   // <1>
use router::router;     // <2>

fn create_routes(url: &str) -> Router {
    let owned_name = format!("{}", url).to_owned();
    router!(
        health: get "/healthz" => health,
        upload: post "/upload/:device_id" => move | request: &mut Request | upload(request, &owned_name), // <3>
        download: get "/download/:id" => download)
}
// end::routes[]

pub fn health(_: &mut Request) -> IronResult<Response> {
    info!("-- Healthz --");
    Ok(Response::with((status::Ok, "OK")))
}
