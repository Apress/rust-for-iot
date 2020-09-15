
/// Copied from the mqtt_service/src/actions/health.rs
use chrono::prelude::*;
use chrono::naive::serde::ts_milliseconds::deserialize as from_milli_ts;    // <1>
use enum_primitive_derive::Primitive;
use serde::{Serialize, Deserialize};
use log::warn;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Primitive, PartialEq)]
pub enum Status {   // <2>
    Green = 0,
    Red = 1,
    Yellow = 2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripheral {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthData {
    pub uuid: Option<Uuid>,
    #[serde(deserialize_with = "from_milli_ts")]    // <3>
    pub timestamp: NaiveDateTime,
    pub status: Status,
    pub user_id: Uuid,
    pub data: String,
    pub peripherals: Vec<Peripheral>
}

// Created for chapter 5
use chrono::{DateTime,Utc};
use num_traits::cast::FromPrimitive;
use crate::database::PgPooled;

impl HealthData {
    pub fn new(uuid: &str, user_id: &str, timestamp: u64, status: u16, msg: &str, ps: Vec<Peripheral>) -> HealthData {
        HealthData {
            uuid: Some(Uuid::parse_str(uuid).unwrap()),
            timestamp: HealthData::convert_time(timestamp),
            status: Status::from_u16(status).unwrap(),
            user_id: Uuid::parse_str(uuid).unwrap(),
            data: msg.to_string(),
            peripherals: ps
        }
    }

    fn convert_time(millis: u64) -> NaiveDateTime {
        use chrono::NaiveDateTime;

        let seconds = (millis / 1000) as i64;
        let nanos = ((millis % 1000) * 1_000_000) as u32;
        //DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
        NaiveDateTime::from_timestamp(seconds, nanos)
    }

    // tag::save[]
    pub fn save(&self, conn: &PgPooled) -> i32 {
        use diesel::{RunQueryDsl, ExpressionMethods};
        use diesel::insert_into;
        use serde_json::{Value};
        use crate::database::schema::health_checks::dsl::*;

        // Converts the application into a Uuid
        let data_json = serde_json::to_value(self).unwrap();

        // save to the database
        match insert_into(health_checks)
            .values((user_id.eq(self.user_id),
                    device_uuid.eq(self.uuid.unwrap()),
                    data.eq(&data_json)))
            .returning(id)
            .get_result::<i32>(&*conn) {
            Ok(val) => {
                val
            },
            Err(e) => {
                warn!("insertion failed, likely invalid reference : {}", e);
                0
            }
        }
    }
    // end::save[]

    // Get the health  check for a particualr user
    pub fn find(user: Uuid, conn: &PgPooled) -> Vec<Uuid> {
        use crate::database::schema::health_checks::dsl::*;
        use diesel::prelude::*;

        health_checks
            .select(device_uuid)
            .filter(user_id.eq(user))
            .load::<Uuid>(&*conn).unwrap()
    }
}