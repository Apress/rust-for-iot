
use log::{info, warn};

use std::fs::File;
use std::io::BufReader;

use exif::{DateTime, Exif, Reader, In, Value, Tag};
use chrono::{Utc, DateTime as DT};

use serde::{Serialize,Deserialize};
use uuid::Uuid;

// tag::struct[]
#[derive(Deserialize, Serialize, Debug)]
pub struct ImageMetaData {          // <1>
    exif_version: Option<f64>,
    x_pixel_dimension: Option<u32>,
    y_pixel_dimension: Option<u32>,
    x_resolution: Option<u32>,
    y_resolution: Option<u32>,
    date_of_image: Option<DT<Utc>>,
    flash: Option<bool>,
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    aperture_value: Option<f64>,
    gps_point: Option<Point>,
    altitude: Option<f64>,
    speed: Option<f64>,
    media_item_id: Uuid,
}

impl ImageMetaData {
    pub fn empty(id: Uuid) -> ImageMetaData {
        ImageMetaData {
            exif_version: None,
            x_pixel_dimension: None,
            y_pixel_dimension: None,
            x_resolution: None,
            y_resolution: None,
            date_of_image: None,
            flash: None,
            make: None,
            model: None,
            exposure_time: None,
            f_number: None,
            aperture_value: None,
            gps_point: None,
            altitude: None,
            speed: None,
            media_item_id: id
        }
    }
}

// This is copied from diesel-geography::GeogPoint, which makes it easier
// for JSON conversion
#[derive(Deserialize, Serialize, Debug)]
pub struct Point {                  // <2>
    pub x: f64, // lon
    pub y: f64, // lat
    pub srid: Option<i32>, // spatial reference identifier
}
// end::struct[]

// tag::parse[]
pub fn parse(media_id: Uuid, file_path: &str) -> Result<ImageMetaData, crate::errors::Error> {
    let file = File::open(file_path).unwrap();
    let reader_result = Reader::new().read_from_container(
        &mut BufReader::new(&file));

    match reader_result {
        Ok(reader) => {
            // create the image data
            Ok(ImageMetaData {                                         // <1>
                exif_version: get_float(&reader, Tag::ExifVersion),
                x_pixel_dimension: get_int(&reader, Tag::PixelXDimension),
                y_pixel_dimension: get_int(&reader, Tag::PixelYDimension),
                x_resolution: get_int(&reader, Tag::XResolution),
                y_resolution: get_int(&reader, Tag::YResolution),
                date_of_image: get_datetime(&reader, Tag::DateTime),
                flash: get_flash(&reader),
                make: get_string(&reader, Tag::Make),
                model: get_string(&reader, Tag::Model),
                exposure_time: get_string(&reader, Tag::ExposureTime),
                f_number: get_string(&reader, Tag::FNumber),
                aperture_value: get_float(&reader, Tag::ApertureValue),
                gps_point: get_geo(&reader),
                altitude: get_float(&reader, Tag::GPSAltitude),
                speed: get_float(&reader, Tag::GPSSpeed),
                media_item_id: media_id
            })
        },
        Err(e) => {
            // THis can happen if there is no EXIF dta
            warn!("Error :: {:?}", e);
            Ok(ImageMetaData::empty(media_id))
        }
    }
}
// end::parse[]

// tag::flash[]
fn get_flash(reader: &Exif) -> Option<bool> {
    match get_string(&reader, Tag::Flash) {
        Some(flash) => {
            Some(flash.starts_with("fired"))
        },
        None => None
    }
}
// end::flash[]

// tag::geo[]
fn get_geo(reader: &Exif) -> Option<Point> {
    let latitude = calculate_pointe(&reader, Tag::GPSLatitude, Tag::GPSLatitudeRef);
    let longitude = calculate_pointe(&reader, Tag::GPSLongitude, Tag::GPSLongitudeRef);
    if latitude == 0.00 || longitude == 0.0 {
        None
    }
    else {
        Some(Point {
            x: longitude,
            y: latitude,
            srid: None
        })
    }
}
// end::geo[]

// tag::pointe[]
fn calculate_pointe(reader: &Exif, dms: Tag, dms_ref: Tag) -> f64 {
    // get latitude
    match reader.get_field(dms, In::PRIMARY) {
        Some(field) => {
            match field.value {
                Value::Rational(ref vec) if !vec.is_empty() => { // <1>
                    let deg = vec[0].to_f64();
                    let min = vec[1].to_f64();
                    let sec = vec[2].to_f64();
                    let ref_factor = calculate_ref(&reader, dms_ref);
                    convert_point(deg, min, sec) * ref_factor
                },
                _ => 0.0
            }
        },
        None => 0.0
    }
}

/// Convert longitude values that are in the western hemisphere or
/// latitude values that are in the southern hemisphere to negative decimal degree values.
/// f64 cause we are going to multiply it
fn calculate_ref(reader: &Exif, dms_ref: Tag) -> f64 {
    match get_string(&reader, dms_ref) {
        Some(field) => {
            match (field.as_ref()) {    // <2>
                "N" => 1.0,
                "S" => -1.0,
                "E" => 1.0,
                "W" => -1.0,
                _ => 1.0
            }
        },
        None => 1.0
    }
}
// end::pointe[]

// tag::pointe2[]
fn convert_point(deg: f64, min: f64, sec: f64) -> f64 {
    (deg + (min / 60.0 ) + (sec / 3600.0 ) )  // <1>
}
// end::pointe2[]

// tag::number[]
fn get_float(reader: &Exif, tag: Tag) -> Option<f64> {
    reader.get_field(tag, In::PRIMARY)
        .and_then(|field| match field.value {
            Value::Rational(ref vec) if !vec.is_empty() => Some(vec[0].to_f64()), // <1>
            _ => None
        })
}

fn get_int(reader: &Exif, tag: Tag) -> Option<u32> {
    reader.get_field(tag,In::PRIMARY)
        .and_then(|field| field.value.get_uint(0) ) // <2>
}

fn get_string(reader: &Exif, tag: Tag) -> Option<String> {
    reader.get_field(tag,In::PRIMARY)
        .and_then(|field| Some(field.value.display_as(tag).to_string()) ) // <3>
}
// end::number[]

// tag::datetime[]
fn get_datetime(reader: &Exif, tag: Tag) -> Option<DT<Utc>> {
    use chrono::offset::TimeZone;               // <1>

    match reader.get_field(tag, In::PRIMARY) {
        Some(field) => {
            let val = field.value.display_as(tag).to_string();          // <2>
            Utc.datetime_from_str(val.as_str(), "%Y-%m-%d %H:%M:%S").ok() // <3>
        },
        None => None
    }
}
// end::datetime[]
