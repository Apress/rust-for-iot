
// From our derived object
use crate::message_capnp::health;
use crate::message_capnp::health::Status;

use crate::actions::health::{Peripheral, HealthData};
use log::{debug, info, error};
use chrono::{DateTime,Utc};

// Traits for our status conversion
use capnp::traits::ToU16;


// TODO : figure this out later!!111
//pub fn deserialize<'a>(buffer: Vec<u8>) -> ::capnp::Result<health::Reader<'a>> {
//    use capnp::serialize::OwnedSegments;
//
//    let deserialized: capnp::message::Reader<OwnedSegments> = capnp::serialize::read_message(
//        &mut buffer.as_slice(),
//        capnp::message::ReaderOptions::new()
//    ).unwrap();
//
//    deserialized.get_root::<health::Reader<'a>>()
//}

//borrowed value does not live long enough
//|     argument requires that `deserialized` is borrowed for `'a`

pub fn convert(health_reader: health::Reader) -> HealthData {
    let peripherals: Vec<Peripheral> = health_reader.get_peripherals().unwrap().iter().map(|p| Peripheral {name: p.get_name().unwrap().to_string()}).collect();

    let data = HealthData::new(health_reader.get_uuid().unwrap(),
                               health_reader.get_timestamp(),
                               health_reader.get_status().unwrap().to_u16(),
                               health_reader.get_msg().unwrap(),
                               peripherals);


    debug!("Data : {:?}", data);
    data
}

pub fn parse(buffer: Vec<u8>) -> HealthData{
    info!("-- Parse Message --");
    // For non package messages
    //let deserialized = capnp::serialize::read_message(
    // Finally, let's deserialize the data for Packed
    let deserialized = capnp::serialize_packed::read_message(
        &mut buffer.as_slice(),
        capnp::message::ReaderOptions::new()
    ).unwrap();

    let health_reader = deserialized.get_root::<health::Reader>().unwrap();

    let peripherals: Vec<Peripheral> = health_reader.get_peripherals().unwrap().iter().map(|p| Peripheral {name: p.get_name().unwrap().to_string()}).collect();

    let data = HealthData::new(health_reader.get_uuid().unwrap(),
                               health_reader.get_timestamp(),
                               health_reader.get_status().unwrap().to_u16(),
                               health_reader.get_msg().unwrap(),
                               peripherals);


    debug!("Data : {:?}", data);
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime,Duration};
    use capnp::traits::FromU16;
    use crate::actions::health::{Status as HealthStatus};

    fn generate_data(time_in_sec: u64, status: u16, msg: &str, peripherals: Vec<&str>) -> Vec<u8> {

        let mut message = ::capnp::message::Builder::new_default();
        {
            let mut health = message.init_root::<health::Builder>();

            health.set_uuid("9cf81814-1df0-49ca-9bac-0b32283eb29b");
            health.set_timestamp(time_in_sec);
            // This could error if the result isnt 1-3 but if thats the case it should
            health.set_status(Status::from_u16(status).unwrap());
            health.set_msg(msg);

            // needs to occur after or you will get "value borrowed here after move" for hte other setters
            let mut health_peripherals = health.init_peripherals(peripherals.len() as u32);
            {
                for i in 0..peripherals.len() {
                    health_peripherals.reborrow().get(i as u32).set_name(peripherals[i]);
                }
            }
        }
        let mut buffer = Vec::new();

        // And actually fill that buffer with our data
        capnp::serialize::write_message(&mut buffer, &message).unwrap();
        buffer
    }

    #[test]
    fn test_parse_with_peripherals() {
        println!("Test Parsing to Health Data");

        let time_in_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() * 1000;

        let given_data = generate_data(time_in_sec, 0, "This message is good", vec!["Camera", "Temp"]);

        let data = parse(given_data);

        assert_eq!(data.uuid.unwrap().to_hyphenated().to_string(), "9cf81814-1df0-49ca-9bac-0b32283eb29b");
        assert_eq!(data.msg.as_str(), "This message is good");
        assert_eq!(data.status, HealthStatus::Green);
        assert_eq!(data.timestamp.timestamp() * 1000, time_in_sec as i64);
    }
}
