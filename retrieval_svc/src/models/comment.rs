
use crate::database::schema::comments;
use serde_derive::Deserialize;
use uuid::Uuid;
use chrono::NaiveDateTime;

// tag::new_comment[]
use diesel::Expression;

// need to bring in the comments module for this to work
#[derive(Insertable, Queryable, PartialEq, Debug)]
#[table_name="comments"]
pub struct NewComment {
    pub body: String,
    pub media_item_id: Uuid,
}
// end::new_comment[]

use juniper::GraphQLObject;
use crate::models::media_data::MediaData;

#[derive(GraphQLObject)]
#[graphql(description = "Media objects for the application")]
#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(MediaData, foreign_key = "media_item_id")]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub media_item_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

use crate::database::schema::comments::dsl::*;
use crate::database::PgPooled;
use log::{info, warn};
use crate::errors::DbResult;

impl Comment {
    pub fn all(media_id: Uuid, conn: &PgPooled) -> Vec<Comment> {
        use crate::database::schema::comments::dsl::*;
        use diesel::prelude::*;

        comments
            .filter(media_item_id.eq(media_id))
            .load::<Comment>(&*conn).unwrap()
    }

    // tag::comment_add[]
    pub fn add(conn: &PgPooled, media_id: Uuid, bod: String) -> i32 {
        use diesel::{RunQueryDsl, ExpressionMethods};
        use diesel::insert_into;

        info!("insert into the comment database");

        match insert_into(comments)
            .values((body.eq(bod), media_item_id.eq(media_id))) // <1>
            .returning(id)                                                  // <2>
            .get_result::<i32>(&*conn) {                                        // <3>
            Ok(val) => {                                                    // <4>
                val
            },
            Err(e) => {
                warn!("insertion failed, likely invalid reference : {}", e);
                -1
            }
        }
    }
    // end::comment_add[]

    // tag::delete[]
    pub fn delete(conn: &PgPooled, comment_id: i32) -> DbResult<u32> {
        use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
        use diesel::delete;
        //use diesel::prelude::;

        let result = delete(comments.filter(id.eq(comment_id)))
            .execute(&*conn);

        match result {
            // Convert since we get it as Usize
            Ok(rows_deleted) => Ok(rows_deleted as u32),
            Err(error) => Err(error),
        }
    }
    // end::delete[]
}

