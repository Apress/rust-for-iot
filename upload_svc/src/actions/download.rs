
use iron::{Request, Response, IronResult};
use iron::prelude::*;
use iron::status;

use log::{info};
use uuid::Uuid;
use router::Router;

pub fn download(req: &mut Request) -> IronResult<Response> {
    info!("-- Download --");
    let id = find_id(&*req, "id");

    Ok(Response::with((status::Ok, "OK")))
}

fn find_id(req: &Request, id_to_find: &str) -> Uuid {
    let mut id: &str = req.extensions.get::<Router>().unwrap()
        .find(id_to_find).unwrap();
    Uuid::parse_str(id).unwrap()
}