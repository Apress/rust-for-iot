
// for the web services
use iron::{Iron, Request, Response, IronResult, status};
use iron::prelude::Chain;
use router::{Router};
use router::router;
use mount::Mount;

use log::{info};

// For the diesel add

use crate::actions::*;

// Chapter 4 - GraphQL Version
#[cfg(feature = "ch04")]
pub fn start(   server: &str, port: u16, auth_server: &str, database_url: &str) {
    // tag::juniper_import[]
    use juniper_iron::{GraphQLHandler, GraphiQLHandler, PlaygroundHandler};
    use juniper::{EmptyMutation, FieldResult};
    use crate::graphql::context::{Context, context_factory, Root, Mutations};
    use eventsourcing::eventstore::OrgEventStore;
    // end::juniper_import[]

    // mounts the router to the
    let mut mount = Mount::new();

    // Api Routes we will setup and support
    let router = create_routes();
    mount.mount("/api", router);

    // GraphQL
    // tag::juniper[]
    let playground = PlaygroundHandler::new("/graph"); // <1>

    let graphql_endpoint = GraphQLHandler::new(     // <2>
        context_factory,
        Root,
        Mutations
    );

    mount.mount("/play", playground);               // <3>
    mount.mount("/graph", graphql_endpoint);        // <4>
    // end::juniper[]

    let mut chain = Chain::new(mount);
    create_links(&mut chain, database_url, auth_server);

    // create the handler and bind it it to a port
    info!("Start Server on {}:{}", server, port);
    Iron::new(chain).http(format!("{}:{}", server, port));
}


// Chapter 5 - GraphQL + CQRS Version
// tag::cqrs[]
#[cfg(feature = "cqrs")]
pub fn start(   server: &str, port: u16, auth_server: &str, database_url: &str,
                event_store_host: &str, event_store_port: u16,                      // <1>
                event_store_user: String, event_store_pass: String) {

    use juniper_iron::{GraphQLHandler, GraphiQLHandler, PlaygroundHandler};
    use juniper::{EmptyMutation, FieldResult};
    use crate::graphql::context::{Context, context_factory, Root, Mutations};
    use eventsourcing::eventstore::OrgEventStore;

    // mounts the router to the
    let mut mount = Mount::new();

    // Api Routes we will setup and support
    let router = create_routes();
    mount.mount("/api", router);

    // need to pass in the end point we want in the mount below
    // no subscription URL
    let playground = PlaygroundHandler::new("/graph");

    // Setup for Org EventStore
    let event_store = OrgEventStore::new_with_auth(event_store_host, event_store_port,  // <2>
                                                   event_store_user, event_store_pass);

    let graphql_endpoint = GraphQLHandler::new(
                                                            context_factory,
                                                            Root,
                                                            Mutations{org_event_store: event_store} // <3>
    );
// end::cqrs[]

    mount.mount("/play", playground);           // <3>
    mount.mount("/graph", graphql_endpoint);    // <4>

    let mut chain = Chain::new(mount);
    create_links(&mut chain, database_url, auth_server);

    // create the handler and bind it it to a port
    info!("Start Server on {}:{}", server, port);
    Iron::new(chain).http(format!("{}:{}", server, port));
}

#[cfg(feature = "cqrs")]
fn create_routes() -> Router {
    router!(
        health: get "/healthz" => health,
        add_media_data: put "/media/add" => media_data::add)
}

#[cfg(feature = "ch04")]
// tag::create_routes[]
fn create_routes() -> Router {
    router!(
        health: get "/healthz" => health,
        // tag::comment[]
        add_comment: put "/comment/add/:media_item_id" => comment::add,
        // end::comment[]
        add_media_data: put "/media/add" => media_data::add)
}
// end::create_routes[]


// Regular Web Endpoints
#[cfg(feature = "ch04")]
pub fn start(server: &str, port: u16, auth_server: &str, database_url: &str) {

    // Routes
    let chain = Chain::new(health_routes());
    let mut api_chain = Chain::new(api_routes());
    create_links(&mut api_chain, database_url, auth_server);

    // Mounts the router
    let mut mount = Mount::new();

    mount.mount("/", chain);
    mount.mount("/api", api_chain);

    // create the handler and bind it it to a port
    info!("Start Server on {}:{}", server, port);
    Iron::new(mount).http(format!("{}:{}", server, port));
}


// So that we know we are using Postgres.
use iron_diesel_middleware::{DieselMiddleware, DieselReqExt};
type DieselPg = DieselMiddleware<diesel::pg::PgConnection>;

#[cfg(feature = "ch04")]
fn create_links(chain: &mut Chain, url: &str, auth_server: &str) {
    // Create the middleware for the diesel
    let diesel_middleware: DieselPg = DieselMiddleware::new(url).unwrap();

    // link the chain
    chain.link_before(diesel_middleware);
}


#[cfg(feature = "ch05")]
fn create_links(chain: &mut Chain, url: &str, auth_server: &str) {
    use crate::authorization::AuthorizationCheck;

    // Create the middleware for the diesel
    let diesel_middleware: DieselPg = DieselMiddleware::new(url).unwrap();

    // link the chain
    chain.link_before(diesel_middleware);
}

#[cfg(feature = "auth")]
// tag::auth_links[]
fn create_links(chain: &mut Chain, url: &str, auth_server: &str) {
    use crate::authorization::AuthorizationCheck;

    // Create the middleware for the diesel
    let diesel_middleware: DieselPg = DieselMiddleware::new(url).unwrap();

    // Authorization tier
    let auth_middleware = AuthorizationCheck::new(auth_server);

    // link the chain
    chain.link_before(auth_middleware);
    chain.link_before(diesel_middleware);
}
// end::auth_links[]

pub fn health(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn health_routes() -> Router {
    router!(
        health: get "/healthz" => health
    )
}

fn api_routes() -> Router {
    router!(
        comment: post "/comment/add/:media_item_id" => comment::add,
        media: post "/media/add" => media_data::add
    )
}
