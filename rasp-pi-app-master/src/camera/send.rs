use crate::errors::HttpResult;

use log::{warn, info};
use std::fs::File;
use std::io::Read;

const PATH : &str = "api/upload";  // <1>

pub(super) fn send_to_backend(device_id: &str, url: &str, file_name: &String, full_path: &String) -> Result<bool, Box<std::error::Error>> {
    use reqwest::blocking::{multipart, Client};

    // Our multipart form
    let form = multipart::Form::new()       // <2>
        // And a file...
        .file(file_name.clone(), full_path.as_str())?;  // <3>

    // Send the request
    let client = Client::new();
    let res = client.post(format!("{}/{}/{}", url, PATH, device_id).as_str())   // <4>
        .multipart(form)
        .send()?;               // <5>
    info!("Status: {}", res.status());

    if res.status() == 200 {    // <6>
        Ok(true)
    }
    else {
        Ok(false)
    }
}

pub(super) async fn send_to_backend_async(device_id: &str, url: &str, file_name: &String, full_path: &String) -> Result<bool, Box<std::error::Error>> {
    use futures_util::{future, stream};

    let name = get_filename(file_name);
    println!("full_path :: {:?}", full_path);
    println!("file_name :: {:?}", file_name);
    println!("name :: {:?}", name);

    // Get the file and send over as bytes
    let file = std::fs::read(full_path);

    // Check if we have the file, if we dont its gone for some reason
    // just delete it from the database then, in actuality you could do some error state messaging
    // instead
    match file {
        Ok(f) => {
            // need to set the mime type
            let part = reqwest::multipart::Part::bytes(f)   // <2>
                // TODO alter although only file exension matters
                .file_name(name)
                .mime_str("video/mp4")?;

            let form = reqwest::multipart::Form::new()  // <3>
                .part("file", part);

            let client = reqwest::Client::new();

            info!("Sending >>> {:?}", format!("{}/{}/{}", url, PATH, device_id).as_str());

            let res = client
                .post(format!("{}/{}/{}", url, PATH, device_id).as_str())   // <4>
                .multipart(form)
                .send()             // <5>
                .await?;

            if res.status() == 200 {    // <6>
                Ok(true)
            } else {
                warn!("Status: {}", res.status());
                Ok(false)
            }
        },
        Err(e) => {
            warn!("Error Getting the File {:?}", e);
            Ok(true)
        }
    }
}

fn get_filename(filename: &String) -> String {
    if filename.contains("/") {
        let x: Vec<&str> = filename.split("/").collect();
        x.last().unwrap().to_string()
    } else {
        filename.to_string()
    }
}
