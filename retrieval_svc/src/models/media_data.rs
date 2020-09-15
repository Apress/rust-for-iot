
use crate::database::schema::media_datas;
use crate::database::schema::comments;

use chrono::NaiveDateTime;
use serde_derive::Deserialize;
use crate::database::{MediaEnum, LocationEnum, MediaAudienceEnum};
use diesel::Expression;
use uuid::Uuid;

// tag::media_data[]
use crate::models::metadata::{Image,Video};
use crate::database::schema::image_metadatas;

// NewMediaData has to have Deserialize/Clone to work with bodyparser
// #[derive(Debug, Deserialize, Clone)]
#[derive(Insertable, Debug, Deserialize, Clone)]
#[table_name="media_datas"]
pub struct NewMediaData{
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub size: i32,
    pub device_id: Uuid
}
// end::media_data[]


use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "Media objects for the application")]
#[derive(Queryable, Identifiable, Debug, Eq, PartialEq)]
pub struct MediaData {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub device_id: Uuid,
    pub size: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


use crate::database::schema::media_datas::dsl::*;
use crate::database::PgPooled;
use log::{debug};
use crate::errors::DbResult;

impl MediaData { }

// tag::impl_media[]
impl NewMediaData {
    // adding the self: &Self to make it a method instead of associated ufction
    // https://doc.rust-lang.org/reference/items/associated-items.html
    pub fn add(self: &Self, conn: &PgPooled) {
        use diesel::insert_into;
        use diesel::RunQueryDsl;

        insert_into(media_datas)
            .values(self)
            //.get_result(&*conn)
            .execute(&*conn)
            .expect("Insertion of new media error");
    }
}
// end::impl_media[]