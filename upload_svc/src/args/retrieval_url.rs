

use clap::Arg;

pub const HELP: &str = "The url to the retrieval service] address to bind to";
pub const LONG_HELP: &str = "\
The address the server will attempt to bind to";
pub const LONG: &str = NAME;
pub const NAME: &str = "retrieval_url";
pub const SHORT: &str = "r";
pub const DEFAULT_VALUE: &str = "http://localhost:3010";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "RETRIEVAL_URL";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .short(SHORT)
        .long(LONG)
        .env(VALUE_NAME)
        .value_name(VALUE_NAME)
        .required(TAKES_VALUE)
        .help(HELP)
        .long_help(LONG_HELP)
        .default_value(DEFAULT_VALUE)
}
