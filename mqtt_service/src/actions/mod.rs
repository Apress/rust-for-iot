
pub mod config;
pub mod recording;
pub mod health;
pub mod health_bytes;

use iron::Request;
use uuid::Uuid;

fn get_uuid_value(req: &Request, name: &str ) -> Uuid {
    let uuid = get_value(req, name);
    Uuid::parse_str(uuid.as_str()).unwrap()
}

fn get_value(req: &Request, name: &str ) -> String {
    use router::Router;

    let mut val: &str = req.extensions.get::<Router>().unwrap()
        .find(name).unwrap();
    val.to_string()
}
