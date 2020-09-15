
use clap::Arg;

pub const HELP: &str = "The Client ID for Auth0 ";
pub const LONG_HELP: &str = "\
The Client ID for Auth0";
pub const LONG: &str = NAME;
pub const NAME: &str = "auth_client_id";
pub const SHORT: &str = "i";
pub const DEFAULT_VALUE: &str = "rsc1qu5My3QZuRPZHp5af5S0MBUcD7Jb";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "AUTH_CLIENT_ID";

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
