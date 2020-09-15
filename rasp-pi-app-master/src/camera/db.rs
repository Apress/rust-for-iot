// tag::sql[]
use rusqlite::{params, Connection, OpenFlags, Result, MappedRows};
use time::Timespec;
use chrono::{DateTime,Utc};
use crate::errors::{DbResult, MyResult};
use log::{warn, error, info};

#[derive(Debug)]
pub(super) struct VideoEntry {
    file_name: String,
    uploaded: bool,
    retry_count: i32,
    upload_time: Option<Timespec>,
    recording_start: Timespec,
    recording_end: Timespec,
}

const PATH : &str = "rust-iot.db";

pub(super) fn initialize() -> bool {
    let conn = create_conn(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

    let size = conn.execute(
        "CREATE TABLE IF NOT EXISTS video_entry (
                  file_name         TEXT PRIMARY KEY,
                  uploaded          INTEGER,
                  retry_count       INTEGER NOT NULL,
                  upload_time       TEXT NULL,
                  recording_start   TEXT NOT NULL,
                  recording_end     TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    if size == 1 {
        true
    }
    else {
        false
    }
}

fn create_conn(flags: OpenFlags) -> Result<Connection> {
   Connection::open_with_flags(
        PATH,
        flags
    )
}
// end::sql[]

// tag::add[]
pub fn add(name: &str, start: DateTime<Utc>, end: DateTime<Utc>)  {
    let conn = create_conn(OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();

    let start_ts = Timespec {
        sec: start.timestamp_millis(),
        nsec: start.timestamp_subsec_nanos() as i32,
    };

    let end_ts = Timespec {
        sec: end.timestamp_millis(),
        nsec: end.timestamp_subsec_nanos() as i32,
    };

    let result = conn.execute(
        "INSERT INTO video_entry (file_name, recording_start, recording_end, uploaded, retry_count)
                  VALUES (?1, ?2, ?3, 0, 0)",
        params![name, start_ts, end_ts],
    );

    match result {
        Ok(_) => {
            info!("Added {:?} to database", name);
        },
        Err(e) => {
            error!("Error Trying to insert into database :: {:?}", e);
        }
    }
}
// end::add[]

// tag::upload[]
fn mark_uploaded(name: String) -> bool {
    let conn = create_conn(OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();

    let size_result = conn.execute(
        "UPDATE video_entry
         Set uploaded = 1
         Where name = ?1",
        params![name]
    );

    // Determine the result
    match size_result {
        Ok(size) => {
            if size > 0 {
                info!("Marked {:?} as uploaded", name);
                true
            }
            else {
                false
            }
        },
        Err(_) => {
            false
        }
    }
}
// end::upload[]

// This increments our counter
// tag::inc[]
fn increment(name: String, current_count: i32) -> bool {
    let conn = create_conn(OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();

    let size_result = conn.execute(
        "UPDATE video_entry
         Set uploaded = 0,
         retry_count = ?1
         Where name = ?2",
        params![current_count + 1, name]
    );

    // Determine the result, of course not mcuh one can do with it
    match size_result {
        Ok(size) => {
            if size > 0 {
                true
            }
            else {
                false
            }
        },
        Err(_) => {
            false
        }
    }
}
// end::inc[]

// DIDNT work
// stmt.query_map(params![], |row| { // <2>
// ...
// })?.into_iter().map(|x| entries.push(x.unwrap()));
// tag::send[]
// only implements Iterator and not IntoIterator
pub(super) fn retrieve_entries(device_id: &str, url: &str, dir: &str) -> DbResult<Vec<VideoEntry>> {
    let conn = create_conn(OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    // Get the non uploaded ones
    let mut stmt = conn.prepare("SELECT file_name, recording_start, recording_end, uploaded, retry_count From video_entry Where uploaded = 0")?;     // <1>
    let mut entries = Vec::new();
    let rows = stmt.query_map(params![], |row| {
        Ok(VideoEntry {
            file_name: row.get(0)?,
            recording_start: row.get(1)?,
            recording_end: row.get(2)?,
            uploaded: row.get(3)?,
            upload_time: None,
            retry_count: row.get(4)?,
        })
    })?;
    for row in rows {
        entries.push(row.unwrap());
    }

    Ok(entries)
}

pub(super) async fn send_to_server(entries: Vec<VideoEntry>, device_id: &str, url: &str, dir: &str) -> DbResult<()> {
    use std::fs::remove_file;

    // Entries
    for video_entry in entries {
        info!("Upload : Video Entry : {:?}", video_entry);

        let full_path = format!("{}/{}", dir, &video_entry.file_name);
        let file_name = video_entry.file_name.clone();

        // Send to the backend
        match super::send::send_to_backend_async(device_id, url, &file_name, &full_path).await {    // <3>
            Ok(value) => {
                if value == true {
                    mark_uploaded(video_entry.file_name);           // <4>
                    // There is a chance this wasn't marked correctly
                    remove_file(&full_path).is_ok();                // <5>
                } else {
                    increment(video_entry.file_name, video_entry.retry_count);  // <6>
                }
            },
            Err(e) => {
                warn!("Error sending the video {:?}", e);
                increment(video_entry.file_name, video_entry.retry_count);      // <7>
            }
        };;
    }
    Ok(())
}
// end::send[]