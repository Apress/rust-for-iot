
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use log::{debug};

pub fn read(file_str: &str) -> Vec<u8> {
    let full = shellexpand::tilde(file_str).into_owned();
    debug!("Reading ... {:?}", full);
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
