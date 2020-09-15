
use clap::Arg;

pub const HELP: &str = "The event store port";
pub const LONG_HELP: &str = "\
The event store port";
pub const LONG: &str = NAME;
pub const NAME: &str = "event_store_port";
pub const SHORT: &str = "x";
pub const DEFAULT_VALUE: &str = "1113";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "EVENT_STORE_PORT";

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
