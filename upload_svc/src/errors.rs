use error_chain::*;
use std::result;
// We can define our error chains in here
//https://docs.rs/error-chain/0.12.0/error_chain/
error_chain! {  // <1>
    foreign_links {
        Io(::std::io::Error);
        Exif(::exif::Error);
//        Mp4(::mp4parse::Error);
    }

    errors {
        NoMetaData
        NoMatchingParser
        SaveMetadata
        Http
        Mp4Parse(phase: ::mp4parse::Error) {
            description("parsing error")
            display("parsing error: {:?}", phase)
        }
//        mp4parse::Error(phase: ValidationError) {
//            description("Error for validating the JWT")
//            display("Error with validation : {:?}", phase)
//        }
    }

//    impl From<::mp4parse::Error> for Error {
//        fn from(err:::mp4parse::Error) -> Error {
//            let kind = err.into_error_kind();
//            Error::from_kind(ErrorKind::Nom(kind))
//        }
//    }
}

use mp4parse::Error as Mp4Error;

// Couple custom errors
pub type MyResult<T> = result::Result<T, Error>; // <2>

pub type HttpResult<T> = result::Result<T, Error>; // <2>

pub type ParseResult<T> = result::Result<T, Error>; // <2>
