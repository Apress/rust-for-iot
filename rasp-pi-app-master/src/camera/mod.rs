mod db;
mod send;
mod video;

use log::{debug,info,error};

// tag::ch09[]
use crate::manager::{FaceTx};

const VIDEO_DIR: &str = ".";

pub fn run_video_capture(mut face_tx: FaceTx) {
    use tokio::task;

    debug!("Start spawn process ..");

    task::spawn_blocking(move || {
        // I want to see me
        debug!("Spawning ...");
        match video::run_face_detect(face_tx, false) {
            Err(err) => {
                error!("Error processing the face :: {:?}", err);
            },
            Ok(_) => {}
        }
    });        
}
// end::ch09[]

// tag::db[]
use uuid::Uuid;

const HOURLY: u64 = 60 * 60;
const BI_MINUTE: u64 = 60 * 2;
const EVERY_MINUTE: u64 = 60 * 1;

// Send our videos hourly to the cloud
pub fn hourly_upload(device_id: String, url: String) {
    use tokio::time::{Duration, interval_at, Instant};
    info!("Hourly Upload for Device : {:?}", device_id);

    db::initialize();
    tokio::spawn(async move {
        // 1 hour duration
        let mut interval = interval_at(Instant::now(), Duration::from_secs(EVERY_MINUTE));
        loop {
            interval.tick().await;
            info!("Upload to the Server");
            let entries = db::retrieve_entries(device_id.as_str(), url.as_str(), VIDEO_DIR);
            info!("Received entries : {:?}", entries);
            match db::send_to_server(entries.unwrap(), device_id.as_str(), url.as_str(), VIDEO_DIR).await {
                Ok(_) => {},
                Err(e) => {
                    error!("Error Sending to the Server {:?}", e);
                }
            }
        }
    });
}
// end::db[]