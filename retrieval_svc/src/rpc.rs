
use log::{debug, info, warn};

use crate::message_capnp::{health,process_update};
use crate::message_capnp::process_update::value;

// Capnp items
use capnp::message::{Builder, HeapAllocator, TypedReader};
use capnp::{serialize_packed,Error};
use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp, pry};

// For threading
// Might need to replace with this later: https://github.com/rust-lang-nursery/futures-rs
// use futures::{Future, Stream};
use futures::{AsyncReadExt, FutureExt, StreamExt, TryFutureExt};
use futures::task::LocalSpawn;
use futures::future;

// for our connection str
//use postgres::{NoTls, Client};
//use r2d2_postgres::PostgresConnectionManager;

pub mod server {
    use super::*;
    use diesel::pg::PgConnection;
    use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
    use crate::database::PgPooled;
    pub type PgPool = Pool<ConnectionManager<PgConnection>>;

    // tag::value[]
    struct ValueImpl {  // <1>
    value: bool
    }

    impl ValueImpl {
        fn new(value: bool) -> ValueImpl {
            ValueImpl { value: value }
        } // <2>
    }

    impl process_update::value::Server for ValueImpl {
        fn read(&mut self,
                _params: process_update::value::ReadParams, // <3>
                mut results: process_update::value::ReadResults) // <4>
                -> Promise<(), Error>
        {
            debug!("Read the Result");
            results.get().set_value(self.value);    // <5>
            Promise::ok(())
        }
    }
    // end::value[]

    // tag::process_update[]
    struct ProcessUpdateImpl { // <1>
    database_pool: PgPool
    }

    impl ProcessUpdateImpl {
        fn new(db: &str) -> ProcessUpdateImpl {
            ProcessUpdateImpl {
                database_pool: ProcessUpdateImpl::establish_connection_pool(db)
            }
        }

        fn get_db(&self) -> PgPooled {
            self.database_pool.clone().get().unwrap()
        }

        fn establish_connection_pool(database_url: &str) -> PgPool {
            use crate::errors::ResultExt;
            use crate::errors::DbResult;

            let manager = ConnectionManager::<PgConnection>::new(database_url);

            // Get the pooled connection manager
            // unrecoverable fail
            Pool::new(manager).expect("Failed creating connection pool")
        }
    }

    impl process_update::Server for ProcessUpdateImpl {     // <2>
    fn call(&mut self,                                  // <3>
            params: process_update::CallParams,         // <4>
            mut results: process_update::CallResults)   // <5>
            -> Promise<(), Error> {
        info!("** received a request for process update");

        let eval = persist( pry!(pry!(params.get()).get_update()),       // <6>
                            self.get_db(),
                            None);

        Promise::from_future(async move {
            let passed = { if eval.await? >= 0 {true} else {false}};
            info!("Evaluate future ... {}", passed);

            results.get().set_passed(
                value::ToClient::new(ValueImpl::new(passed)).into_client::<::capnp_rpc::Server>()); // <7>
            Ok(())
        })

    }
    }
    // end::process_update[]

    // tag::evaluate[]
    use capnp::primitive_list;
    fn persist( health_reader: health::Reader,
                conn: PgPooled,
                params: Option<primitive_list::Reader<f64>>)
                -> Promise<i32, Error>
    {

        use crate::models::health_check::{HealthData,Peripheral};
        use capnp::traits::ToU16;

        let peripherals: Vec<Peripheral> = health_reader.get_peripherals().unwrap().iter().map(|p| Peripheral {name: p.get_name().unwrap().to_string()}).collect(); // <1>

        let data = HealthData::new(health_reader.get_uuid().unwrap(),
                                    health_reader.get_user_id().unwrap(),
                                   health_reader.get_timestamp(),
                                   health_reader.get_status().unwrap().to_u16(),
                                   health_reader.get_msg().unwrap(),
                                   peripherals);

        let id = data.save(&conn);   // <2>

        Promise::ok(id)     // <3>
    }
    // end::evaluate[]

    // This will accept the socket connections for the server to accept
    // Eventually might be replaced with :
    // https://github.com/rust-lang-nursery/futures-rs
    // This is being used by CapnProto-RPC
    // tag::server[]
    pub fn start(host: &str, port: u16, database: &str) -> Result<(), ::capnp::Error> {
        use std::net::ToSocketAddrs;

        info!("Start RPC Server : {}:{}", host, port);
        let socket_address = format!("{}:{}", host, port);
        let socket_addr = socket_address.to_socket_addrs().unwrap().next().expect("could not parse address");

        // spawns a local pool
        let mut exec = futures::executor::LocalPool::new();
        let spawner = exec.spawner();

        let result: Result<(), Box<dyn std::error::Error>> = exec.run_until(async move {
            // Set up the socket
            let listener = async_std::net::TcpListener::bind(&socket_addr).await?;         // <1>

            // Set the server that we implemented
            let pu =
                process_update::ToClient::new(ProcessUpdateImpl::new(database)).into_client::<::capnp_rpc::Server>();  // <2>

            // listen on the incoming socket
            let mut incoming = listener.incoming();
            while let Some(socket) = incoming.next().await {  // <3>
                // unwrap it
                let socket = socket?;
                socket.set_nodelay(true)?;
                let (reader, writer) = socket.split();

                let network =
                    twoparty::VatNetwork::new(reader, writer,
                                              rpc_twoparty_capnp::Side::Server, Default::default());    // <4>

                let rpc_system = RpcSystem::new(Box::new(network), Some(pu.clone().client)); // <5>

                // Spawns the local object
                spawner.spawn_local_obj(
                    Box::pin(rpc_system.map_err(|e| warn!("error: {:?}", e)).map(|_|())).into()).expect("spawn")

            }
            Ok(())
        });

        info!(" Done with Run Server");
        result.expect("rpc");
        Ok(())
    }
    // end::server[]
}
