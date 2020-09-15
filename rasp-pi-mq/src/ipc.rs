use packet_ipc::{AsIpcPacket, Client, Error, IpcPacket, Packet, Server, ConnectedIpc};
use futures::future::Future;
use std::{thread, time};
use log::info;

use crate::file;
use crate::actions::recording::RecordingType;

// tag::ipc[]
const file_location: &str = "/tmp/pi_upc_server_name_file";     // <1>

pub fn send(recording: RecordingType) {
    // lock server and use
    let mut server_tx = init().expect("Failed to accept connection");   // <2>

    let payload = format!("{:?}", recording);               // <3>
    let x = payload.as_bytes();

    server_tx
        .send(&vec![Packet::new(std::time::SystemTime::now(), x.to_vec())]) // <4>
        .expect("Failed to send");
}

/**
 * Initialize the server connection to be used.
 */
pub fn init<'a>() -> Result<ConnectedIpc<'a>, Error> {
    // Start up the server
    let server = Server::new().expect("Failed to create server");       // <5>
    let server_name = server.name().clone();
    file::write(file_location.to_string(), &server_name);

    info!("Server Name :: {:?}", server_name);

    return server.accept()
}
// end::ipc[]