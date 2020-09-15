use error_chain::*;
use std::result;

use rumqtt::error::ConnectError;

// We can define our error chains in here
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        MqttError(phase: ConnectError) {
            description("Error for MQTT Connection")
            display("Error with mqtt connection : {:?}", phase)
        }
    }
}

// Couple custom errors
pub type MyResult<T> = result::Result<T, Error>;
pub type MqttResult<T> = result::Result<T, ConnectError>;
