use crate::database::*;

// tag::comment[]
table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    health_checks (id) {
        id -> Int4,
        device_uuid -> Uuid,
        data -> Jsonb,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
// end::comment[]


// tag::manual[]
table! {
    // Manual edition
    use diesel::sql_types::{Integer, Array, Text, Bool, Timestamp, Nullable, Uuid}; // <1>
    use super::Media_Enum_Map; // <2>
    use super::Location_Enum_Map;
    use super::Media_Audience_Enum_Map;

    media_datas (id) {
        id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        media_type -> Media_Enum_Map, // <3>
        location -> Text,
        location_type -> Location_Enum_Map,
        device_id -> Uuid,
        // uses MediaAudienceEnumMapping?? TODO? Fix this
        media_audience_type -> Array<Media_Audience_Enum_Map>, // <4>
        size -> Integer,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
// end::manual[]

table! {
    use diesel::sql_types::*;
    use bigdecimal::BigDecimal;

    video_metadatas (id) {
        id -> Int4,
        video_duration -> Nullable<Numeric>,
        video_width -> Nullable<Numeric>,
        video_height -> Nullable<Numeric>,
        video_codec -> Nullable<Varchar>,
        audio_track_id -> Nullable<Numeric>,
        audio_codec -> Nullable<Varchar>,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    // needed for our Geography item
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;
    // added for this one
    use bigdecimal::BigDecimal;

    image_metadatas (id) {
        id -> Int4,
        exif_version -> Nullable<Numeric>,
        x_pixel_dimension -> Nullable<Int4>,
        y_pixel_dimension -> Nullable<Int4>,
        x_resolution -> Nullable<Int4>,
        y_resolution -> Nullable<Int4>,
        date_of_image -> Nullable<Timestamp>,
        flash -> Nullable<Bool>,
        make -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        exposure_time -> Nullable<Varchar>,
        f_number -> Nullable<Varchar>,
        aperture_value -> Nullable<Numeric>,
        location -> Geography,
        altitude -> Nullable<Numeric>,
        speed -> Nullable<Numeric>,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// tag::join[]
joinable!(comments -> media_datas (media_item_id));
// end::join[]
joinable!(image_metadatas -> media_datas (media_item_id));
joinable!(video_metadatas -> media_datas (media_item_id));

allow_tables_to_appear_in_same_query!(
    comments,
    health_checks,
    image_metadatas,
    media_datas,
    spatial_ref_sys,
    video_metadatas,
);

