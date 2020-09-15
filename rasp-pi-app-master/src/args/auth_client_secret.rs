
use clap::Arg;

pub const HELP: &str = "The Secret for Auth0 ";
pub const LONG_HELP: &str = "\
The Secret for Auth0";
pub const LONG: &str = NAME;
pub const NAME: &str = "auth_client_secret";
pub const SHORT: &str = "t";
pub const DEFAULT_VALUE: &str = "C4YMZHE9dAFaEAysRH4rrao9YAjIKBM8-FZ4iCiN8G-MJjrq7O0alAn9qDoq3YF6";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "AUTH_CLIENT_SECRET";

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
