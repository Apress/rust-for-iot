// tag::import[]
use std::{thread, time::Duration};
use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

use opencv::videoio::{VideoCapture, VideoWriter};
use opencv::objdetect::CascadeClassifier;
use opencv::calib3d::UndistortTypes;
// end::import[]

use crate::manager::{FaceTx};
use super::db;

use chrono::{DateTime, Utc};
use log::{debug,info,warn,error};
use std::sync::{Arc,Mutex};
use futures::SinkExt;

const ONE_MIN_IN_SECONDS: f64 = 60.0;
const MINUTES_BETWEEN_IMAGES: i64 = 1;
const MINUTES_BETWEEN_NEW_VIDEO: f64 = 1.0;
const FPS_FACTOR: f64 = 0.5;

const START: &str = "Start";
const STOP: &str = "Stop";

const haar_cascade_file: &str = "./haarcascade_frontalface_alt.xml";
const file_location: &str = "/tmp/pi_upc_server_name_file";
const media_dir: &str = "media";

// can run a Haar Cascade on the image
pub fn run_face_detect(mut face_tx: FaceTx, show_window: bool) -> opencv::Result<()> {
    info!("Start face detection ...");

    // tag::window[]
    let window = "video capture";
    if show_window {
        highgui::named_window(window, 1)?;
    }
    // end::window[]

    // example of retrieving from a file
    // tag::cam_init[]
    let mut cam =  videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?;  // 0 is the default camera
    // Open the camera
    let opened = cam.is_opened()?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    // end::cam_init[]

    // Face detection
    // tag::face[]
    if !std::path::Path::new(&haar_cascade_file).exists() { panic!("Haar Cascade is needed!"); }
    let mut face = objdetect::CascadeClassifier::new(&haar_cascade_file)?;
    // end::face[]

    //let mut start_time: DateTime<Utc> = Utc::now();
    let mut last_face_detect_time: Option<DateTime<Utc>> = None;

    // Setting up the writer
    // Different Props
    // https://docs.rs/opencv/0.8.0/opencv/videoio/index.html
    // tag::size[]
    let width = cam.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap();
    let height = cam.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap();
    let fps = cam.get(videoio::CAP_PROP_FPS).unwrap() * FPS_FACTOR;
    info!("Camera Settings : {:?} x {:?}, with {:?} fps", width, height, fps);

    // Size for the output
    let size = core::Size {
        width: width as i32,
        height: height as i32
    };
    // end::size[]

    // Start first writer
    // tag::start[]
    let mut writer = videoio::VideoWriter::new(create_file_name().as_str(), fourcc(), fps, size, true)?;
    // Frames per second, for a minute, times how many minutes
    let frames_per_file = fps * ONE_MIN_IN_SECONDS * MINUTES_BETWEEN_NEW_VIDEO;
    info!("Will create {:?} frames per a file", frames_per_file);
    // end::start[]

    // The monitor for the control
    // tag::monitor[]
    let recording_execute: Arc<Mutex<String>> = Arc::new(Mutex::new(START.to_string()));    // <1>
    let recording_monitor = recording_execute.clone();                              // <2>

    monitor_controls(recording_monitor);    // <3>

    match handle_video(face, show_window,                                 // <4>
                writer, frames_per_file, last_face_detect_time,
                face_tx, size, window, fps,
                cam, &recording_execute) {
        // Only returns if there is an error
        Ok(_) => {},
        Err(e) => {
            error!("Error handling the video :: {:?}", e);
        }
    };
    // end::monitor[]
    Ok(())
}

// tag::client[]
fn monitor_controls(recording_monitor: Arc<Mutex<String>>) {
    use std::{str,fs};
    use packet_ipc::{AsIpcPacket, Client, Error, IpcPacket, Packet, Server};
    use std::{thread, time};

    let one_second = time::Duration::from_secs(1);

    thread::spawn(move || {                 // <1>
        loop {
            thread::sleep(one_second);      // <2>

            let server_name = fs::read_to_string(file_location.to_string()).unwrap();   // <3>

            let client_res = Client::new(server_name.clone()).map(|mut cli| {    // <4>
                let mut packets = vec![];
                // Pushes a packet
                // This contains the information received frm the clent
                let val = cli.recv(1);      // <5>
                info!("Push a packet! :: {:?}", val);
                // can keep receiving till you get None bpackets back
                packets.push(val);                              // <6>
                packets
            });

            match client_res {
                Ok(res) => {
                    info!("Await ...");
                    let res: Result<Vec<_>, Error> = res.into_iter().collect();         // <7>
                    let res = res.expect("Failed to get packets");

                    let packets = res[0].as_ref().expect("No message"); // <8>
                    let data = packets[0].data();
                    info!(">> {:?}", str::from_utf8(data).unwrap());
                    let value = str::from_utf8(data).unwrap();

                    let mut guard = recording_monitor.lock().unwrap();  // <9>
                    *guard = value.to_string();                                            // <10>
                },
                Err(e) => {}
            }
        }
    });
}
// end::client[]

async fn send(tx: &mut FaceTx, show: bool) {
    if let Err(_) = tx.send(show).await {
        info!("receiver dropped");
        return;
    }
}

fn is_record(command: &Arc<Mutex<String>>) -> bool{
    let c = &*command.lock().unwrap();
    let cmd = c.as_str();
    match cmd {
        START => true,
        STOP => false,
        _ => false
    }
}

fn dont_record(command: &Arc<Mutex<String>>) -> bool {
    let c = &*command.lock().unwrap();
    let cmd = c.as_str();
    match cmd {
        START => false,
        STOP => true,
        _ => true
    }
}


fn handle_video(mut face: CascadeClassifier, show_window: bool,
                mut writer: VideoWriter, frames_per_file: f64, mut last_face_detect_time: Option<DateTime<Utc>>,
                mut face_tx: FaceTx, size: core::Size, window: &str, fps: f64,
                mut cam: VideoCapture, recording_execute: &Arc<Mutex<String>>) -> opencv::Result<()> {

    let mut is_recording: bool = true;

    // tag::start2[]
    let mut i : f64 = 0f64;
    let mut start_time: DateTime<Utc> = Utc::now();
    let mut file_name = "none".to_string();

    loop {
        // end::start2[]
        // tag::cam_read[]
        let mut frame = Mat::default()?;
        cam.read(&mut frame)?;

        // Sleeps in case there is no screen coming
        if frame.empty().unwrap() {
            debug!("frame empty? camera not iontialized?");
            thread::sleep(Duration::from_secs(50));
            continue;
        }
        // end::cam_read[]

        // Converts an image from one color space to another.
        // Converts the frame to gray
        // tag::gray[]
        let mut gray_frame = Mat::default()?; // <1>
        imgproc::cvt_color(
            &frame,
            &mut gray_frame,
            imgproc::COLOR_BGR2GRAY,
            0
        )?;

        // Resizes an image.
        // The function resize resizes the image src down to or up to the specified size.
        let mut reduced = Mat::default()?;  // <2>
        imgproc::resize(
            &gray_frame,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0
            },
            0.25f64,                // <3>
            0.25f64,
            imgproc::INTER_LINEAR
        )?;
        // end::gray[]

        // Detects objects of different sizes in the input image. The detected objects are returned as a list
        // of rectangles.
        // tag::detect[]
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(        // <1>
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30
            },
            core::Size {
                width: 0,
                height: 0
            }
        )?;
        // end::detect[]

        // tag::face_detect[]
        // If any faces are detected send the face detection here
        if faces.len() == 0 {                   // <1>
            //face_tx.send(false).unwrap();
            send(&mut face_tx, false);
        } else {
            send(&mut face_tx, true);
        }
        // end::face_detect[]

        // Objects returned in list of rectangles
        for face in &faces {
            debug!("face {:?}", face);
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0f64, -1f64, -1f64, -1f64),
                1,
                8,
                0
            )?;
        }

        // tag::save_image[]
        if !faces.is_empty() {                // <1>
            // Send that the face was detected
            send(&mut face_tx, true);

            // is this our first face in a minute, save an image as well.
            // this part is only needed for start up
            debug!("IMAGE: Check for a face every {:?} minutes", MINUTES_BETWEEN_IMAGES);
            match last_face_detect_time {
                Some(last_face_time) => {
                    let next_image_time = last_face_time + chrono::Duration::minutes(MINUTES_BETWEEN_IMAGES);    // <2>
                    info!("IMAGE: Last Time: {:?} / Next Detect Time: {:?}", last_face_time, next_image_time);
                    if Utc::now() > next_image_time {                                               // <3>
                        info!("IMAGE: Save image");
                        if save_image(&frame).is_err() {             // <4>
                            warn!("Error saving the image to the file system");
                        }
                        // reset the time
                        last_face_detect_time = Some(Utc::now());
                    }
                },
                None => {                       // <5>
                    // first time detected save it off
                    info!("IMAGE: Save first image");
                    if save_image(&frame).is_err() {
                        warn!("Error saving the image to the file system");
                    }
                    last_face_detect_time = Some(Utc::now());
                }
            };
        }
        // end::save_image[]
        else {
            // No face detection
            send(&mut face_tx, false);
        }


        // add the date to the bottom left corner
        //let point = Point::new((original_image_cols / 1.7) as i32, (original_image_rows / 4.2) as i32)
        // tag::draw_time[]
        let now: DateTime<Utc> = Utc::now();
        let date = format!("TS: {}", now.format("%d/%m/%Y %H:%M.%S"));  // <1>

        // scalar is the color / true : for bottom left origin
        let point = core::Point::new(10, 20); // <2>
        imgproc::put_text(&mut frame,
                          date.as_str(),
                          point,
                          highgui::QT_FONT_NORMAL,                      // <3>
                          0.8,
                          // BGR .. reverse of RGB
                          core::Scalar::new(0., 0., 255., 0.),  // <4>
                          2,
                          imgproc::LINE_8,
                          false)?;                              // <5>
        // end::draw_time[]

        // Choose whether to record the video file
        record(&mut writer, frames_per_file, size, fps, &recording_execute, &mut i,
               &mut start_time, &mut is_recording, &mut frame, &mut file_name);

        // tag::window_show[]
        if show_window {
            highgui::imshow(window, &frame)?;
            // can be used for a bit of a delay
            if highgui::wait_key(10)? > 0 {
                break;
            }
        }
        // end::window_show[]
    }
    Ok(())
}

// tag::save_image_func[]
use crate::errors::MyResult;

fn save_image(frame: &Mat) -> MyResult<bool> {
//    use rexiv2::Metadata;

    info!("IMAGE: Save image");
    let image_name = create_image_name();   // <1>
    let mut params = opencv::types::VectorOfint::new();
    params.push(opencv::imgcodecs::IMWRITE_JPEG_OPTIMIZE); // <2>
    // Need gexiv installed on the computer ofr thi to work
    opencv::imgcodecs::imwrite(image_name.as_str(), &frame, &params).unwrap(); // <3>
    match opencv::imgcodecs::imwrite(image_name.as_str(), &frame, &params) {
        Ok(_) => {
            // now save the meta data
            // TODO Fix docker image then reactivate it
//            if let Ok(meta) = Metadata::new_from_path(image_name.as_str()) {
//                // defined in main.rs
//                meta.set_tag_string("Exif.Image.Make", crate::APP_NAME);    // <4>
//                meta.save_to_file(image_name).unwrap();             // <5>
//            }
        },
        Err(e) => {
            error!("Error writing the file out :: {:?}", e);
            return Ok(false);
        }
    };

    Ok(true)
}
// end::save_image_func[]

// Detail for if and when we should record the video
#[cfg(feature = "ch10")]
// tag::save_video[]
fn record(writer: &mut VideoWriter, frames_per_file: f64, size: core::Size, fps: f64,
          i: &mut f64, start_time: &mut DateTime<Utc>,
          mut frame: &mut Mat, file_name: &mut String) -> opencv::Result<()> {
    // Write it locally with the file
    writer.write(frame)?;       // <1>

    if *i == frames_per_file {
        info!("Created File : from {:?} to {:?}", start_time, Utc::now());
        writer.release()?;          // <2>
        *file_name = create_file_name();
        *writer = videoio::VideoWriter::new(&file_name.as_str(), fourcc(), fps, size, true)?;   // <3>
        *start_time = Utc::now();
        *i = 1f64;
    }
    else {
        *i = 1f64 + *i;             // <4>
    }

    Ok(())
}
// end::save_video[]

#[cfg(feature = "full")]
// tag::record[]
fn record(writer: &mut VideoWriter, frames_per_file: f64, size: core::Size, fps: f64,
          recording_execute: &Arc<Mutex<String>>, i: &mut f64, start_time: &mut DateTime<Utc>,
          is_recording: &mut bool, mut frame: &mut Mat, file_name: &mut String) -> opencv::Result<()> {
    // Currently recording, and no stop command
    if *is_recording
        && is_record(&recording_execute) {  // <1>
        // Release and restart the file
        // Write it locally with the file
        writer.write(frame)?;

        if *i == frames_per_file {
            info!("Created File : from {:?} to {:?}", start_time, Utc::now());
            writer.release()?;
            *file_name = create_file_name();
            *writer = videoio::VideoWriter::new(&file_name.as_str(), fourcc(), fps, size, true)?;
            db::add(&file_name, *start_time, Utc::now());
            *start_time = Utc::now();
            *i = 1f64;
        }
        else {
            *i = 1f64 + *i;
        }
    }
    // is recording but received a stop command
    // so set the is_recording to false and write a release file
    else if *is_recording
        && dont_record(&recording_execute) {        // <2>
        // Stop the recording and save to the file
        *is_recording = false;
        writer.release()?;
        db::add(&file_name, *start_time, Utc::now());
    }
    // not currently recording, but needs to start again
    else if !*is_recording
        && is_record(&recording_execute) {      // <3>
        *is_recording = true;
        *start_time = Utc::now();
        *file_name = create_file_name();
        *writer = videoio::VideoWriter::new(&file_name.as_str(), fourcc(), fps, size, true)?;
        *i = 1f64;
    }
    else {
        warn!("Not supported.");
    }

    Ok(())
}
// end::record[]

// tag::fourcc[]
fn fourcc() -> i32 {
    videoio::VideoWriter::fourcc('m' as i8, 'p' as i8,'4' as i8,'v' as i8).unwrap()
}
// end::fourcc[]

fn create_image_name() -> String {
    let now: DateTime<Utc> = Utc::now();

    let date = now.format("%d.%m.%Y_%H.%M.%S");

    format!("{}/image-{}.jpg", media_dir, date)
}

// tag::file_name[]
fn create_file_name() ->  String {
    let now: DateTime<Utc> = Utc::now();

    let date = now.format("%d.%m.%Y_%H.%M.%S");

    format!("{}/video-{}.mp4", media_dir, date)
}
// end::file_name[]