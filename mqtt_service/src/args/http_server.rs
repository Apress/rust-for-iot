
use clap::Arg;

pub const HELP: &str = "The server address for the http to bind to";
pub const LONG_HELP: &str = "\
The address the server will attempt to bind to for the http server";
pub const LONG: &str = NAME;
pub const NAME: &str = "http_server";
pub const SHORT: &str = "h";
pub const DEFAULT_VALUE: &str = "localhost";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "HTTP_SERVER_ADDR";

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
