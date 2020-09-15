
use clap::Arg;

pub const HELP: &str = "The Client Key";
pub const LONG_HELP: &str = "The Client Key";
pub const LONG: &str = NAME;
pub const NAME: &str = "client-key";
pub const SHORT: &str = "k";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "CLIENT_KEY";
pub const DEFAULT_VALUE: &str = "~/book_certs/EmqttIot.key";

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
