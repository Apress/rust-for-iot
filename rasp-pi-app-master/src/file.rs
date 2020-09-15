
use std::fs::File;
use std::io::prelude::*;

use log::{debug};

pub fn read(file_str: &str) -> Vec<u8> {
    debug!("Setting ... {:?}", file_str);
    let mut f = File::open(file_str).unwrap();
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
