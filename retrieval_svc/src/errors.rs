use error_chain::*;
use std::result;

use alcoholic_jwt::ValidationError;

// We can define our error chains in here
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        JwtValidation(phase: ValidationError) {
            description("Error for validating the JWT")
            display("Error with validation : {:?}", phase)
        }
    }
}

//impl From<ValidationError> for ErrorKind {
//    fn from(v: ValidationError) -> Self {
//        ErrorKind::JwtValidation(v)
//    }
//}

// Couple custom errors
pub type Success = result::Result<(), Error>;
//pub type UserResult = result::Result<String, crate::errors::ErrorKind>;
pub type UserResult = result::Result<String, ValidationError>;
pub type MyResult<T> = result::Result<T, Error>;
pub type DbResult<T> = result::Result<T, diesel::result::Error>;