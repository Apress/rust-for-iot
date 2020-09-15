
use log::{info, debug};

// tag::cmd[]
use uuid::Uuid;
use eventsourcing::prelude::*;      // <1>
use eventsourcing::Result;
use eventsourcing::eventstore::EventStore;
use serde_derive::{Serialize, Deserialize};
use crate::database::PgPooled;

const DOMAIN_VERSION: &str = "1.0";

pub enum CommentCommand {   // <2>
AddComment { body: String, media_item_id: Option<Uuid> },
    DeleteComment { id: Uuid }
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version(DOMAIN_VERSION)]                   // <3>
#[event_source("events://iot-rust/comment")]
pub enum CommentEvent {
    CommentAdded { id: Uuid, body: String, media_item_id: Option<Uuid> },   // <4>
    CommentDeleted { id: Uuid }
}
// end::cmd[]

// tag::event_from[]
// Used to convert from one type to another
impl From<&CommentCommand> for CommentEvent {    // <1>
    fn from(source: &CommentCommand) -> Self {   // <2>
        match source {
            CommentCommand::AddComment{body, media_item_id} => {    // <3>
                CommentEvent::CommentAdded{
                    id: Uuid::new_v4(),                        // <4>
                    body: body.to_string(),
                    media_item_id: *media_item_id
                }
            },
            CommentCommand::DeleteComment { id } => CommentEvent::CommentDeleted { id: *id } // <5>
        }
    }
}
// end::event_from[]

// tag::state[]
#[derive(Debug, Clone)]
pub struct CommentState {   // <1>
pub body: String,
    pub media_item_id: Option<Uuid>,
    pub generation: u64
}

impl AggregateState for CommentState { // <2>
fn generation(&self) -> u64 {
    self.generation
}
}

pub struct CommentAggregate;
impl Aggregate for CommentAggregate {       // <3>
type Event = CommentEvent;              // <4>
type Command = CommentCommand;
    type State = CommentState;

    // Apply events to state, producing new state.
    fn apply_event(state: &Self::State, event: &Self::Event) -> Result<Self::State> {    // <5>
        info!("Apply event");
        // needs to implement the event on the state itself
        unimplemented!()
    }

    /// 2. Handle commands, producing a vector of outbound events, likely candidates for publication.
    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {   // <6>
        info!("Handle Command");
        // validate

        // Only if validation passes return the events for it
        Ok(vec![cmd.into()])    // <7>
    }
}
// end::state[]

// tag::dispatch[]
#[derive(Dispatcher)]
#[aggregate(CommentAggregate)]
pub struct CommentDispatcher;
// end::dispatch[]

// tag::executor[]
pub trait Executor {        // <1>
    fn run(&self, pool: &PgPooled) -> bool;
}

impl Executor for CommentEvent {
    fn run(&self, pool: &PgPooled) -> bool {
        debug!("Execute our comment {:?}", &self);
        match &self {
            CommentEvent::CommentAdded{ id, body, media_item_id }=> {   // <2>
                comment_add(id, body, media_item_id, pool)
            },
            _ => false
        }
    }
}

fn comment_add(id: &Uuid, body: &String, media_item_id: &Option<Uuid>, pool: &PgPooled) -> bool {    // <3>
    use std::{thread, time};
    use crate::models::comment::Comment;;

    info!("Execute our add Comment '{}' / {:?}", body, media_item_id);
    info!("----> {}", id);
    // TODO Store the comment data
    Comment::add(pool, media_item_id.unwrap(), body.to_string()) > 0 // <4>
}
// end::executor[]