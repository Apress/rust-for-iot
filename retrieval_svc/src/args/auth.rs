
use clap::Arg;

pub const HELP: &str = "The auth server address";
pub const LONG_HELP: &str = "\
The Auth Server Address";
pub const LONG: &str = NAME;
pub const NAME: &str = "auth";
pub const SHORT: &str = "a";
pub const DEFAULT_VALUE: &str = "rustfortheiot.auth0.com";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "AUTH_ADDR";

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
