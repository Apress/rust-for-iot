
use mp4parse as mp4;
use mp4parse::{MediaContext,Track,SampleEntry};
use std::fs::File;
use std::io::{Cursor, Read};

use log::{warn,info};

use serde::{Serialize,Deserialize};
use uuid::Uuid;

// tag::structure[]
#[derive(Deserialize, Serialize, Debug)] // <1>
pub struct VideoMetaData {              // <2>
    video_duration: Option<u64>,
    video_width: Option<u32>,
    video_height: Option<u32>,
    video_codec: Option<String>,
    audio_track_id: Option<u32>,
    audio_codec: Option<String>,
    media_item_id: Uuid,
}

impl VideoMetaData {                    // <3>
    fn new(id: Uuid) -> VideoMetaData {
        VideoMetaData {
            video_duration: None,
            video_width: None,
            video_height: None,
            video_codec: None,
            audio_track_id: None,
            audio_codec: None,
            media_item_id: id,
        }
    }
}
// end::structure[]

///
/// This will parse out the metadata content for the Video Content
///

// TODO use the error_chain
// tag::init[]
pub fn parse(uuid: Uuid, file_path: &str) -> Result<VideoMetaData, crate::errors::Error> {
    use crate::errors::ErrorKind::Mp4Parse;

    let mut file = File::open(file_path)?;
    let mut context = MediaContext::new();      // <1>

    match mp4parse::read_mp4(&mut file, &mut context) { // <2>
        Ok(_) => {
            // return the meta data
            Ok(create_meta_data(uuid, context)) // <3>
        },
        Err(e) => {
            warn!("Error reading Mp4 : {:?}", e);
            // return a blank object
            Ok(VideoMetaData::new(uuid))
        }
    }
}
// end::init[]

// tag::parse[]
fn create_meta_data(uuid: Uuid, context: MediaContext) -> VideoMetaData {
    info!("Movie extend box : {:?}", context.mvex);

    let mut vmc = VideoMetaData::new(uuid);
    check_tracks(context.tracks,  vmc)
}

fn check_tracks(tracks: Vec<Track>, mut vmc: VideoMetaData) -> VideoMetaData {
    for track in tracks {                                       // <1>
        match track.track_type {
            mp4::TrackType::Video => {                                  // <2>
                vmc.video_duration = Some(track.duration.unwrap().0);
                // Reference is here if not we will get a borrow err below
                match &track.tkhd {
                    Some(tkhd) => {
                        vmc.video_width = Some(tkhd.width);
                        vmc.video_height = Some(tkhd.height);
                    },
                    None => {}
                };
                vmc.video_codec = retrieve_codec(&track);
            },

            mp4::TrackType::Audio => {                                  // <3>
                vmc.audio_track_id = Some(track.track_id.unwrap());
                vmc.audio_codec = retrieve_codec(&track);
            },

            mp4::TrackType::Metadata | mp4::TrackType::Unknown => {}
        };
    }

    vmc
}
// end::parse[]

// tag::codec[]
fn retrieve_codec(track: &Track) -> Option<String> {
    match &track.stsd {
        Some(stsd) => {
            match stsd.descriptions.first() {
                Some(v) => {
                    match v {
                        mp4::SampleEntry::Video(v) => {
                            Some(stringify!(v.codec_type).to_string()) // <1>
                        },
                        mp4::SampleEntry::Audio(v) => {
                            Some(stringify!(v.codec_type).to_string()) // <2>
                        },
                        _ => {
                            None
                        }
                    }
                },
                None => {
                    None
                }
            }
        },
        None => {
            None
        },
    }
}
// end::codec[]