
use eventstore::{ Connection, EventData };
//use futures::executor::block_on;
use log::{info, debug};
use tokio::prelude::*;
use std::error::Error;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use diesel::pg::PgConnection;
use crate::database::PgPooled;

// tag::start[]
#[tokio::main]
pub async fn start(database_url: &str, host: &str, port: u16) {
    use std::net::ToSocketAddrs;

    let url = format!("{}:{}", host, port);
    debug!("Connect to : {:?} and database : {:?}", url, database_url);
    info!("ES Connect :: {:?}", url);

    // Create socket adddress.
    let endpoint = url.to_socket_addrs().unwrap().next().unwrap();  // <1>

    // start up our connector
    let connection = Connection::builder()  // <2>
        .single_node_connection(endpoint)
        .await;

    // Subscription
    subscription(&connection, database_url).await;        // <3>
}
// end::start[]

// Copied from ch02/db_iron_example/src/db/connectrs
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_conn(database_url: &str) -> PgPooled {
    // Get the pooled connection manager
    let pool_conn_mgr: PgPool = establish_connection_pool(database_url);
    let pooled_conn_mgr: PgPooled = pool_conn_mgr.get().expect("Error creating a connection pool");

    pooled_conn_mgr
}

pub fn establish_connection_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Get the pooled connection manager
    Pool::new(manager).expect("Failed creating connection pool")
}

// tag::sub[]
// Constants
pub const STREAM_ID: &str = "my_comment";
const GROUP_NAME: &str = "server_group";

async fn subscription(connection: &Connection, database_url: &str)
                      -> Result<(), Box<dyn Error>> {
    use crate::domain::Executor;    // <1>
    info!("Start Subscription ...");

    // Can do programmatically or create here
    let _ = connection                              // <2>
        .create_persistent_subscription(STREAM_ID.clone(), GROUP_NAME)
        .execute()
        .await?;

    let (mut sub_read, mut sub_write) = connection      // <3>
        .connect_persistent_subscription(STREAM_ID.clone(), GROUP_NAME)
        .execute();
println!("ZIFT 1");
    // Database Connection
    let pool = establish_conn(database_url);
println!("ZIFT 2");
    // Iterate over to send the event
    while let Some(event) = sub_read.read_next().await {            // <4>
        println!("ZIFT 3");
        let originalEvent = event.inner.get_original_event();       // <5>
        let mut data: crate::domain::CommentEvent = originalEvent.as_json().unwrap();   // <6>
        info!("Data From Stream >> {:?}", data);
        data.run(&pool);                 // <7>

        sub_write.ack_event(event).await;       // <8>
    }

    Ok(())
}

// end::sub[]