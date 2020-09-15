

controller_imports!();

use iron_diesel_middleware::DieselReqExt;
use crate::database::PgPooled;

use uuid::Uuid;

use log::{info, error};
use crate::models::comment::Comment;

// tag::params_use[]
use params::{Params, Value, Map}; // <1>
// end::params_use[]
use std::ops::Deref;

pub fn add(req: &mut Request) -> IronResult<Response> {
    info!("-- add comments --");
    // tag::params_init[]
    let map = req.get_ref::<Params>().unwrap(); // <2>
    // end::params_init[]

    error!("URL:: {:?}", map);
    let comment: String = get_comment(&map);
    let media_item_id = find_media_id(&req);

    info!("Insert '{}' into {}", comment, media_item_id);
    let conn: PgPooled = req.db_conn();
    Comment::add(&conn, media_item_id, comment);

    Ok(Response::with((status::Ok, "OK")))
}

// tag::params[]
const COMMENT_FIELD: &str = "comment";

fn get_comment(map: &Map) -> String {
    let x: &str = match map.find(&[COMMENT_FIELD]).unwrap() { // <3>
        Value::String(s) => s.as_ref(),
        _ => "none",
    };
    String::from(x)
}
// end::params[]

// tag::router[]
fn find_media_id(req: &Request) -> Uuid {
    let id: &str = req.extensions.get::<Router>().unwrap()
        .find("media_item_id").unwrap();
    Uuid::parse_str(id).unwrap()
}
// end::router[]

//pub fn add() {
//    let bod = "joseph";
//    let conn = establish_connection_manager();
//    use schema::comments::dsl::*;
//
//    insert_into(comments)
//        .values(body.eq(bod))
//        .returning(id)
//        .get_result::<i32>(&*conn).unwrap();
//}
////
//// Create a comment for a connection returning the id that is created
////
//pub fn create_comment<'a>(conn: &PgConnection, bod: &'a str) -> i32 {
//    use schema::comments::dsl::*;
//
//    insert_into(comments)
//        .values(body.eq(bod))
//        .returning(id)
//        .get_result::<i32>(&*conn).unwrap()
//}
