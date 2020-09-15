use error_chain::*;
use std::result;

// We can define our error chains in here
error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Db(::rusqlite::Error);
//        Req(::reqwest::error::Error);
    }
}

// Couple custom errors
pub type MyResult<T> = result::Result<T, Error>;
pub type DbResult<T> = result::Result<T, Error>;
pub type HttpResult<T> = result::Result<T, Error>;

