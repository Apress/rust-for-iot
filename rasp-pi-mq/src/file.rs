use std::fs::File;
use std::io::prelude::*;

use log::{debug, info};

pub fn read(file_str: &str) -> Vec<u8> {
    // Convert for us
    let full = shellexpand::tilde(file_str).into_owned();
    debug!("Setting ... {:?}", full);

    let mut f = File::open(full).unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    match f.read_to_end(&mut buffer) {
        Ok(_val) => {}
        Err(e) => {
            panic!("Unrecoverable, you need to have the proper certs : {:?}", e)
        },
    };
    buffer
}

pub fn write(file_str: String, value: &String) -> std::io::Result<()> {
    info!("Setting ... {:?}", file_str);
    let mut file = File::create(file_str)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}