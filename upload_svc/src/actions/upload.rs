use iron::{Request, Response, IronResult};
use iron::prelude::*;
use iron::status;

use std::io::{self, Write};
use multipart::mock::StdoutTee;
use multipart::server::{Multipart, Entries, SaveResult};
use multipart::server::save::{DataReader, SaveDir, SavedData, SaveBuilder};

// Save to file
use std::fs::File;
use std::ffi::OsStr;
use std::path::Path;

use log::{debug, info, error};

use crate::parsers::FileMetaData;

//use rusoto_core::Region;
//use rusoto_s3::{S3, S3Client, PutObjectRequest};

use uuid::Uuid;

// ResultExt needed for the macro
use crate::errors::{ResultExt, MyResult};
use crate::errors::ErrorKind::{Http, SaveMetadata};
use crate::parsers::image::ImageMetaData;
use crate::parsers::video::VideoMetaData;

use serde::{Serialize,Deserialize};

//const PATH: &str = "~/rust-iot/temp";
const PATH: &str = "/Users/jnusairat/rust-iot/temp";

const ADD_MEDIA_DATA: &str = "/api/media/add";

// tag::type[]
#[derive(Debug)]
enum FileType {
    Image,
    Video,
    Unknown
}
// end::type[]

// tag::struct[]
// Mimics the Enums in Retrieval Svc
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MediaEnum {
    Image,
    Video,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LocationEnum {
    S3,
    Local
}

#[derive(Deserialize, Serialize, Debug)]
struct FileUpload {
    id: Uuid,
    name: String,
    media_type: MediaEnum,
    location: String,
    location_type: LocationEnum,
    size: u64,
    image_data: Option<ImageMetaData>,
    video_data: Option<VideoMetaData>,
    device_id: Uuid
}
// end::struct[]

// tag::struct_impl[]
impl FileUpload {
    fn new_image(uuid: Uuid, name: String, location: String,
                 location_type: LocationEnum, size: u64,
                 image: ImageMetaData, device_id: Uuid) -> FileUpload {
        FileUpload {
            id: uuid,
            name: name,
            media_type: MediaEnum::Image,
            location: location,
            location_type: location_type,
            size: size,
            image_data: Some(image),
            video_data: None,
            device_id: device_id
        }
    }

    fn new_video(uuid: Uuid, name: String, location: String,
                 location_type: LocationEnum, size: u64,
                 video: VideoMetaData, device_id: Uuid) -> FileUpload {
        FileUpload {
            id: uuid,
            name: name,
            media_type: MediaEnum::Video,
            location: location,
            location_type: location_type,
            size: size,
            image_data: None,
            video_data: Some(video),
            device_id: device_id
        }
    }

    fn new(uuid: Uuid, name: String, location: String,
           location_type: LocationEnum, size: u64,
           device_id: Uuid) -> FileUpload {
        FileUpload {
            id: uuid,
            name: name,
            media_type: MediaEnum::Unknown,
            location: location,
            location_type: location_type,
            size: size,
            image_data: None,
            video_data: None,
            device_id: device_id
        }
    }
}
// end::struct_impl[]

// tag::upload[]
use params::{Params, Value, Map};

const DEVICE_ID_FIELD: &str = "device_id";

pub fn upload(req: &mut Request, retrieval_svc_url: &str)
              -> IronResult<Response> {
    use router::Router;
    let mut id: &str = req.extensions.get::<Router>().unwrap()
        .find(DEVICE_ID_FIELD).unwrap();

    let device_id = Uuid::parse_str(id).unwrap();
    info!("Upload for Device ID :: {}", device_id);

    save_multipart(req, retrieval_svc_url, device_id)
}

// end::upload[]

// tag::save_multipart[]
fn save_multipart(req: &mut Request, retrieval_svc_url: &str, device_id: Uuid)
    -> IronResult<Response>{
    if let Some(entries) = req.extensions.get::<Entries>() {
        debug!("{:?}", entries);
    } else {
        debug!("Not a multipart request");
    }

    // In case you want to eventually make the location S3
    let location_type = LocationEnum::Local;            // <1>

    // Save the entries for multipart requests
    if let Some(entries) = req.extensions.get::<Entries>() { // <2>
        debug!("-- Multi Part Requests");
        match save_entries(&entries, retrieval_svc_url, location_type,
                           device_id) {
            Ok(status) => {                                      // <3>
                info!("Succeeded : {:?}", status);
                Ok(Response::with((status::Ok, "OK")))
            },
            Err(e) => {                                 // <4>
                error!("error saving the file {}", e);
                Ok(Response::with(status::InternalServerError))
            },
        }
    } else {
        Ok(Response::with(status::NotFound)) // <5>
    }
}
// end::save_multipart[]

// tag::process[]
use std::path::PathBuf;

fn save_entries(entries: &Entries, retrieval_svc_url: &str,
                location_type: LocationEnum, device_id: Uuid)
                    -> MyResult<()> {

    for (key, value) in &entries.fields {   // <1>
        // the file part will show up as "file" the others will be the field name
        if key.as_ref().eq("file") {
            info!("{} / {:?}", key, value);
            for field in value {
                let filename = &field.headers.filename; // <2>
                let size = field.data.size();                       // <3>
                let data = &field.data;                     // <4>

                save_data_matcher(data, filename, size,
                                  retrieval_svc_url, &location_type,
                                  device_id);    // <5>
            }
        }
    }
    Ok(())
}
// end::process[]

// tag::save_data_matcher[]
fn save_data_matcher(data: &SavedData, filename: &Option<String>,
                     size: u64, retrieval_svc_url: &str,
                     location_type: &LocationEnum, device_id: Uuid) {
    // tag::uuid[]
    let uuid = Uuid::new_v4();
    debug!("File to save :: {:?} to {}", filename, uuid);

    match save_data(&uuid, &data, &filename,uuid.to_hyphenated().to_string()) {
    // end::uuid[]
        Ok((file_saved_name, file_meta_data)) => {
            info!("Saved with MetaData :: {:?}", file_meta_data);
            send_to_retrieval_svc(retrieval_svc_url, filename,  size,
            location_type.clone(), file_saved_name,
            uuid, file_meta_data, device_id);
        }
        Err(error) => {
            error!("Encountered an error saving the data {}", error);
        }
    }
}
// end::save_data_matcher[]

// tag::send[]
use reqwest::blocking::Client;
use http::status::StatusCode;

///
/// Send the meta data to the retrieval data service.
///
fn send_to_retrieval_svc(url: &str, filename: &Option<String>, size: u64,
                         location_type: LocationEnum,
                         file_saved_name: String, uuid: Uuid,
                         file_meta_data: FileMetaData,
                         device_id: Uuid) -> MyResult<StatusCode> {
    let name = filename.to_owned().unwrap_or("none".to_string());   // <1>

    let location = format!("/api/media/add/{}", file_saved_name); // <2>

    let file_upload = match file_meta_data {                       // <3>
        FileMetaData::Image(image) => FileUpload::new_image(uuid,
                        name, location, location_type, size, image, device_id),
        FileMetaData::Video(video) => FileUpload::new_video(uuid,
                        name, location, location_type, size, video, device_id),
        _ => FileUpload::new(uuid, name, location, location_type, size, device_id)
    };

    // Create the URL
    let mut add_media = url.to_owned();             // <4>
    add_media.push_str(&ADD_MEDIA_DATA);

    info!("Send HTTP Request {}", url);
    info!("Sending Data :: {:?}", file_upload);

    send_json(add_media.as_str(), file_upload)
}

fn send_json(add_media: &str, file_upload: FileUpload) -> MyResult<StatusCode> {
    let c = Client::new()       // <5>
        .put(add_media)                // <6>
        .json(&file_upload)          // <7>
        .send();
    match c {
        Ok(response) => {
            // TODO : Success is also for 500s we need to do something with that
            // TODO: perhaps retry but as of right now not much we cna do
            info!("Put sucessfully sent: {}", response.status());
            Ok(response.status())
        },
        Err(error) => {
            error!("Error sending : {:?} : error:: {:?}", add_media, error);
            Err(Http.into())
        },
    }
}
// end::send[]

// tag::save_data[]
fn save_data(uuid: &Uuid, data: &SavedData, filename: &Option<String>, file_id: String)
    -> MyResult<(String, FileMetaData)> {
    // Match and handle the type of data we have
    match data {
        SavedData::File(file, bytes) => {   // <1>
            info!("File data");
            debug!("Move file :: {:?}", file);
            save_file_to_file(uuid,
                              retrieve_path(),
                              get_extension_for_name(filename),&file_id, &file)
        },
        SavedData::Text(txt) => {   // <2>
            info!("Text data");
            save_text_to_file(retrieve_path(), &file_id, &txt)
        },
        SavedData::Bytes(byes_data) => {    // <3>
            // Not tested yet
            info!("Byte Data");
            save_byte_to_file(retrieve_path(), &file_id, byes_data)
        }
    }
}

// Strips the extension from the back of a file name
fn get_extension_for_name(filename: &Option<String>) -> String {    // <4>
    match filename {
        Some(name) => {
            let x: Vec<&str> = name.split(".").collect();
            x.last().unwrap().to_string()
        },
        None => "unk".to_string()
    }
}
// end::save_data[]

// tag::parse_metadata[]
use crate::parsers::image::parse as image_parse;    // <1>
use crate::parsers::video::parse as video_parse;
use crate::errors::ParseResult;

fn parse_metadata(uuid: &Uuid, file_type: FileType, path: &str) -> ParseResult<FileMetaData> {
    use crate::errors::ErrorKind::NoMatchingParser;

    info!("Parse Meta Data :: {:?}", file_type);
    match file_type {
        FileType::Image => Ok(FileMetaData::Image(image_parse(uuid.clone(),path)?)), // <2>
        FileType::Video => Ok(FileMetaData::Video(video_parse(uuid.clone(), path)?)),
        // in theory we could try and see if either video or image could parse this
        // since it could be just a bad extension btu correct file headers
        FileType::Unknown => Err(NoMatchingParser.into())
    }
}
// end::parse_metadata[]

// tag::save_file1[]
fn save_file_to_file(uuid: &Uuid, path: String, extension: String, name: &String, file_path: &PathBuf)
    -> MyResult<(String, FileMetaData)> {
    use crate::errors::ErrorKind::NoMetaData;

    info!("Save to file {}, {}, {:?}", path, name, file_path);

    // need to do this in 2 steps so we can have the memory saved longer
    //let file_name = format!("{}", name, extension);
    let path_to_save_name = format!("{}/{}.{}", path, name, extension);
    let path_to_save = Path::new(&path_to_save_name);

    info!("Save from : {:?} to {:?}", file_path, path_to_save);

    std::fs::copy(file_path, path_to_save)
        .chain_err(|| "unable to write to file")?;

    match parse_metadata(uuid, extension_type(extension),
                         path_to_save.to_str().unwrap()) {                     // <1>
        Ok(metadata) => Ok((name.clone(), metadata)),                       // <2>
        Err(err) => Err(NoMetaData.into())               // <3>
    }
}

fn extension_type(extension: String) -> FileType{   // <4>
    debug!("Extension :: {}", extension);
    match extension.to_lowercase().as_str() {
        "tiff" => FileType::Image,
        "jpg" => FileType::Image,
        "jpeg" => FileType::Image,
        "mov" => FileType::Video,
        "mp4" => FileType::Video,
        "m4v" => FileType::Video,
        _ => FileType::Unknown
    }
}
// end::save_file1[]

// tag::save_file2[]
fn save_text_to_file(path: String, name: &String, data: &String) -> MyResult<(String, FileMetaData)> { // <1>
    std::fs::write(format!("{}/{}", path, name), data)
        .chain_err(|| "unable to write to file")?;

    Ok((name.to_string(), FileMetaData::None))
}

fn save_byte_to_file(path: String, name: &String, data: &Vec<u8>) -> MyResult<(String, FileMetaData)> {   // <2>
    use std::fs::write;

    let file_name = format!("{}/{}", path, name);
    write(file_name, data)?;

    Ok((name.to_string(), FileMetaData::None))
}
// end::save_file2[]

fn retrieve_path() -> String {
    use std::env;

    match env::var("PATH_FOR_FILE") {
        Ok(val) => val,
        Err(e) => PATH.to_string()
    }
}

// S3 Sample
//pub fn upload() {
//    // upload this file to S3
//    let mut paths = Vec::new();
//    paths.push(1);
//    let s3_client = S3Client::new(Region::UsEast1);
//    println!("{}", json::stringify(paths));
//
//
//    s3_client.put_object(PutObjectRequest {
//        bucket: String::from("bucket"),
//        key: "@types.json".to_string(),
//        body: Some(json::stringify(paths)),
//        acl: Some("public-read".to_string()),
//        ..Default::default()
//    }).sync().expect("could not upload");
//
//    // analyze the file for size and send to the other service
//}

// do the CreateMultipatUplaodRequest forloarger fles
//
//type StreamingBody = ByteStream;
//
//impl From<Vec<u8>> for ByteStream
//
//fn example(s: String) -> rusoto_s3::StreamingBody {
//    s.into_bytes().into()
//}