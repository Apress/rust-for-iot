

use diesel_derive_enum::DbEnum;
use serde_derive::Deserialize;
//use diesel::Expression;
use juniper::{FieldResult,GraphQLEnum,GraphQLObject};

//#[derive(DbEnum,Debug, Deserialize, Clone, AsExpression)]
// tag::db_enum_custom[]
#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)] //<1>
#[DieselType = "Media_Enum_Map"] //<2>
#[derive(GraphQLEnum)]
pub enum MediaEnum { //<3>
    Image,
    Video,
    Unknown,
}
// end::db_enum_custom[]

//#[derive(DbEnum,Debug, Deserialize, Clone, AsExpression)]
#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)]
#[DieselType = "Location_Enum_Map"]
#[derive(GraphQLEnum)]
// tag::enum[]
pub enum LocationEnum {
    S3,
    Local
}
// end::enum[]

// tag::db_enum[]
#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)]
#[DieselType = "Media_Audience_Enum_Map"]
#[derive(GraphQLEnum)]
pub enum MediaAudienceEnum {
    Personal,
    Friends,
    Family
}
// end::db_enum[]

pub mod schema;

// Use when we need to get the connection passed through in pages.
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
pub type PgPooled = PooledConnection<ConnectionManager<PgConnection>>;
