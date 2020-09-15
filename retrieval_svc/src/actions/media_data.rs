
controller_imports!();

use bodyparser;
use persistent::Read;

use log::{info, warn, error};
use crate::models::media_data::{MediaData, NewMediaData};
use params::{Params, Value, Map};

use uuid::Uuid;

use iron_diesel_middleware::DieselReqExt;

// tag::add[]
pub fn add(req: &mut Request) -> IronResult<Response> {
    info!("-- add media data --");
    let json_body = req.get::<bodyparser::Json>();
    info!(">>>> JSON ::: {:?}", json_body);

    let struct_body = req.get::<bodyparser::Struct<MediaDataAdd>>(); // <1>

    match struct_body {
        Ok(Some(media_data)) => {
            info!("Parsed body:\n{:?}", media_data);
            media_data.save(&req.db_conn());    // <2>
            Ok(Response::with((status::Ok, "OK")))
        },
        Ok(None) => {
            warn!("No body");
            Ok(Response::with(status::BadRequest))      // <3>
        },
        Err(err) => {
            error!("Error parsing meta data :: {:?}", err);
            Ok(Response::with(status::InternalServerError)) // <4>
        }
    }
}
// end::add[]

pub fn get(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

pub fn delete(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

pub fn search(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

// tag::media[]
use crate::database::PgPooled;
use crate::models::metadata::{Image,Video};
use serde_derive::Deserialize;
use crate::database::{MediaEnum, LocationEnum};

#[derive(Deserialize, Debug, Clone)]
pub struct MediaDataAdd {
    pub id: Uuid,
    pub name: String,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub size: i32,
    pub device_id: Uuid,
    pub image_data: Option<Image>, // <1>
    pub video_data: Option<Video>  // <2>
}

#[cfg(feature = "ch02")]
impl MediaDataAdd {
    fn save(self: Self, pool: &PgPooled) {}     // <3>
}
// end::media[]

#[cfg(feature = "full")]
impl MediaDataAdd {
    // Cant do a reference for &self because it will move out of borrowed context error
    fn save(self: Self, pool: &PgPooled) {
        // save the image / video or save the data
        let media_data = NewMediaData {
            id: self.id,
            name: self.name,
            note: None,
            location: self.location,
            size: self.size,
            device_id: self.device_id,
            media_type: self.media_type,
            location_type: self.location_type
        };
        media_data.add(pool);
        // now save either the video or image
        self.image_data.map(| image | image.save(&pool));
        self.video_data.map(| video | video.save(&pool));
    }
}

//
//fn insert_media<'a>(media_data: &'a NewMediaData, conn: crate::database::PgPooled) -> MediaData {
//    use crate::database::schema::media_datas::dsl::*;
//    //use crate::database::schema::media_datas;
//    use crate::database::{MediaEnum, LocationEnum};
//    use diesel::insert_into;
//
//    //let conn: PgConnection = establish_connection();
//    let n = NewMediaData {
//        name: String::from("jose"),
//        note: None,
//        //media_type: Vec<MediaEnum.Audio>,
//        location: String::from("here"),
//        //location_type: Vec<LocationEnum.S3>,
//        size: 32,
//    };
//
//    insert_into(media_datas)
//        .values(&media_data)
//        .get_result(&*conn)
//        .expect("Error saving new post")
//}
//use crate::database::PgPooled;
//fn insert_media<'a>(media_data: &'a NewMediaData, conn: &PgPooled) {
//    use crate::database::schema::media_datas::dsl::*;
//    //use crate::database::schema::media_datas;
//    use crate::database::{MediaEnum, LocationEnum};
//    use diesel::{RunQueryDsl, ExpressionMethods};
//    use diesel::insert_into;
//
//    //let conn: PgConnection = establish_connection();
//    let n = NewMediaData {
//        name: String::from("jose"),
//        note: None,
//        //media_type: Vec<MediaEnum.Audio>,
//        location: String::from("here"),
//        //location_type: Vec<LocationEnum.S3>,
//        size: 32,
//    };
//
//    insert_into(media_datas::table)
//        .values(&media_data)
//        .execute(&*conn)
//        .expect("Error saving new post");
//}
