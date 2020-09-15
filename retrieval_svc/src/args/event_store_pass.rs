
use clap::Arg;

pub const HELP: &str = "The event store pass";
pub const LONG_HELP: &str = "\
The event store pass";
pub const LONG: &str = NAME;
pub const NAME: &str = "event_store_pass";
pub const SHORT: &str = "z";
pub const DEFAULT_VALUE: &str = "changeit";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "EVENT_STORE_PASS";

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
