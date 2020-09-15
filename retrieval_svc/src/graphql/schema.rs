

use juniper::{FieldResult,GraphQLEnum,GraphQLObject,FromInputValue};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::database::{MediaAudienceEnum, MediaEnum, LocationEnum};

use diesel::Expression;
use crate::models::comment::Comment;
use crate::models::media_data::MediaData;
use crate::database::PgPooled;

#[derive(GraphQLObject)]
#[graphql(description = "Media objects with comments")]
pub struct MediaComment {
    pub media: MediaData,
    pub comments: Vec<Comment>,
}

impl MediaComment {
    pub fn new(item: (MediaData, Vec<Comment>)) -> MediaComment {
        MediaComment {
            media: item.0,
            comments: item.1,
        }
    }

    /*
     * Find all media items
     */
    pub fn all(conn: &PgPooled) -> Vec<MediaComment> {
        // imports a bunch of aliases so that we can say media_datas instead of media_datas::table
        use crate::database::schema::media_datas::dsl::*;
        use diesel::prelude::*;

        let medias = media_datas.select((id, name, note, media_type, location, location_type, device_id, size, published, created_at, updated_at))
            .order(created_at.asc()).load::<MediaData>(&*conn).unwrap();

        let comments = Comment::belonging_to(&medias)
            .load::<Comment>(&*conn).unwrap()
            .grouped_by(&medias);

        medias.into_iter().zip(comments)
            .map(|x| MediaComment::new(x))
            .collect::<Vec<MediaComment>>()
    }

    /**
     * Find all media for a particular device id
     */
    pub fn find(id: Uuid, conn: &PgPooled) -> Vec<MediaComment> {
        // imports a bunch of aliases so that we can say media_datas instead of media_datas::table
        use crate::database::schema::media_datas::dsl::*;
        use diesel::prelude::*;

        let medias = media_datas.select((id, name, note, media_type, location, location_type, device_id, size, published, created_at, updated_at))
            .filter(device_id.eq(id))
            .order(created_at.asc()).load::<MediaData>(&*conn).unwrap();

        let comments = Comment::belonging_to(&medias)
            .load::<Comment>(&*conn).unwrap()
            .grouped_by(&medias);

        medias.into_iter().zip(comments)
            .map(|x| MediaComment::new(x))
            .collect::<Vec<MediaComment>>()
    }

}

// tag::schema[]
#[derive(GraphQLObject)]
#[graphql(description = "Media objects for the application")]
pub struct Media {      // <1>
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub media_type: MediaEnum,
    pub location: String,
    pub location_type: LocationEnum,
    pub device_id: Uuid,
    pub media_audience_type: Vec<MediaAudienceEnum>,
    pub size: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject)]
#[graphql(description = "Comments given to the item")]
#[derive(Queryable, PartialEq, Debug)]
pub struct CommentG {        // <2>
    pub id: i32,
    pub body: String,
    pub media_item_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
// end::schema[]
