
//use crate::graphql::schema::Comment;
use crate::database::PgPooled;
use juniper::FieldResult;
use uuid::Uuid;
use iron::{Request, IronResult};
use iron_diesel_middleware::DieselReqExt;

use std::net::SocketAddr;

use log::{info, debug};

use eventsourcing::eventstore::OrgEventStore;

// tag::context[]
pub struct Context {                // <1>
    pub pool: PgPooled              // <2>
}

impl juniper::Context for Context {}    // <3>

pub fn context_factory(req: &mut Request) -> IronResult<Context> {  // <4>
    Ok(Context {
        pool: req.db_conn()                 // <5>
    })
}
// end::context[]

///
/// Query Objects
///
// tag::query[]
use crate::graphql::schema::{CommentG, Media, MediaComment};
use crate::models::media_data::MediaData;
use crate::models::comment::Comment;

pub struct Root;   // <1>

#[cfg(feature = "cqrs")]
#[juniper::object(  // <2>
    Context = Context,
)]
impl Root {        // <3>
    fn all_media(context: &Context) -> Vec<MediaComment> {     // <4>
        MediaComment::all(&context.pool)               // <5>
    }

    fn find(device_id: Uuid, context: &Context) -> Vec<MediaComment> {
        MediaComment::find(device_id, &context.pool)
    }

    fn comments(media_id: Uuid, context: &Context) -> Vec<Comment> {
        let pool = &context.pool;
        Comment::all(media_id, &pool)
    }

    fn health(user_id: Uuid, context: &Context) -> Vec<Uuid> {
        HealthData::find(user_id, &context.pool)
    }
}
// end::query[]

// Start: Chapter5
// tag::cqrs[]
#[cfg(feature = "cqrs")]
pub struct Mutations {
    pub org_event_store: eventsourcing::eventstore::OrgEventStore
}
// end::cqrs[]

#[cfg(feature = "cqrs")]
#[derive(juniper::GraphQLObject)]
pub struct MutationResult {
    success: bool,
    value: Uuid
}
// End: Chapter5

// tag::mutation[]
use crate::models::comment::Comment as CommentDb;

#[cfg(feature = "ch04")]
#[derive(juniper::GraphQLObject)]   // <1>
pub struct MutationResult {         // <2>
    success: bool,
    value: i32
}

#[cfg(feature = "ch04")]
pub struct Mutations;   // <3>

#[juniper::object(  // <4>
    Context = Context,
)]
impl Mutations {    // <5>
    fn add_comment(&self, media_item_id: Uuid, body: String, context: &Context) -> FieldResult<MutationResult> {   // <6>
        // Validate inputs and save user in database...
        info!("Add comment :: {}, {}", media_item_id, body);
        let val = add_comment(self, context, media_item_id, body);
        let result = MutationResult {
            success: true,
            value: val,
        };
        Ok(result)  // <7>
    }

    fn delete_comment(comment_id: i32, context: &Context) -> FieldResult<bool> {
        // Validate inputs and save user in database...
        info!("Del comment :: {}", comment_id);
        let success = match CommentDb::delete(&context.pool, comment_id) {
            Ok(_) => true,
            Err(_) => false
        };
        Ok(success)
    }
}

#[cfg(feature = "ch04")]
fn add_comment(mutations: &Mutations, context: &Context, media_item_id: Uuid, body: String) -> i32 {
    CommentDb::add(&context.pool, media_item_id, body)
}
// end::mutation[]

// tag::cqrs_addcomment[]
#[cfg(feature = "cqrs")]
fn add_comment(mutations: &Mutations, context: &Context, media_item_id: Uuid, body: String) -> Uuid {
    comment_add(&mutations.org_event_store, media_item_id, body.as_str())
}

use crate::domain::{CommentCommand, CommentState, CommentDispatcher, CommentEvent};
use crate::models::health_check::HealthData;

// Send via the CQRS
pub fn comment_add(eventstore: &OrgEventStore, media_id: Uuid, comment: &str) -> Uuid {
    use uuid::Uuid;
    // dispatcher trait
    use eventsourcing::Dispatcher;          // <1>
    // For the event sourcing
    use eventsourcing::eventstore::OrgEventStore;   // <2>

    // You create the command you want to execute
    let command = CommentCommand::AddComment {   // <3>
        body: comment.to_string(),
        media_item_id: Some(media_id),
    };

    // our state, the initial one, aggregate should emit this.
    // this is the returned state that should then be used for subsequent calls
    let state = CommentState {               // <4>
        body: "".to_string(),
        media_item_id: None,
        generation: 0
    };

    // Successful call will return the Event
    debug!("Dispatch ...");
    let res =  CommentDispatcher::dispatch(&state, &command, eventstore.clone(), crate::cqrs::STREAM_ID.clone());  // <5>
    let data = &res[0].as_ref().unwrap().data;      // <6>
    let id = data.get("CommentAdded").unwrap().get("id").unwrap();
    let uuid_created = match id {
        serde_json::Value::String(st) => {
            Some(Uuid::parse_str(st).unwrap())
        },
        _ => { None }
    };

    uuid_created.unwrap() // <7>
}
// end::cqrs_addcomment[]