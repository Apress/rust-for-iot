
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
use tokio::runtime::current_thread;
use futures::{Future, Stream};
use tokio::io::{AsyncRead};

pub mod client {
    use super::*;

    // tag::run_health[]
    pub fn run_health(host: &str, port: u16, buffer: Vec<u8>) -> Result<(), ::capnp::Error> {
        use capnp::serialize::OwnedSegments;

        let deserialized: capnp::message::Reader<OwnedSegments> = capnp::serialize_packed::read_message(    // <1>
            &mut buffer.as_slice(),
            capnp::message::ReaderOptions::new()
        ).unwrap();

        let health = deserialized.get_root::<health::Reader>(); // <2>

        run(host, port, health.unwrap())    // <3>
    }
    // end::run_health[]

    // tag::run[]
    fn run(host: &str, port: u16, health: health::Reader) -> Result<(), ::capnp::Error> {
        // Set up the socket
        use std::net::ToSocketAddrs;

        // Create a socket address
        let socket_address = format!("{}:{}", host, port);  // <1>
        info!(" Start Run Client: {}", socket_address);

        let socket_addr = socket_address.to_socket_addrs().unwrap().next().expect("could not parse address");

        // this is the executor
        // runtime calls the poll on the future until its value is returned
        let mut runtime = ::tokio::runtime::current_thread::Runtime::new().unwrap();    // <2>
        // is a non blocking connect call
        let stream = runtime.block_on(::tokio::net::TcpStream::connect(&socket_addr)).unwrap(); // <3>

        stream.set_nodelay(true)?;
        let (reader, writer) = stream.split();

        let network =
            Box::new(twoparty::VatNetwork::new(reader, std::io::BufWriter::new(writer),
                                            rpc_twoparty_capnp::Side::Client,
                                            Default::default()));       // <4>

        let mut rpc_system = RpcSystem::new(network, None);     // <5>
        let process_update: process_update::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);    // <6>

        // This is just to capture any errors we may have goten and acknowledge htem.
        // spwans the variious tasks?
        runtime.spawn(rpc_system.map_err(|_e| ()));
        {

            // Call was dderived from us using the word call in our interface adding _request
            let mut request = process_update.call_request();        // <7>

            let mut builder = request.get().set_update(health);     // <8>

            let value = request.send().pipeline.get_passed();       // <9>
            let request = value.read_request();                     // <10>
            runtime.block_on(request.send().promise.and_then(|response| {   // <11>
                info!("Response :: {}", pry!(response.get()).get_value());  // <12>
                Promise::ok(())
            }))?;

            info!("Request sent ...");
        }

        Ok(())
    }
    // end::run[]
}
