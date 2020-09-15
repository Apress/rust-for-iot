
use clap::Arg;

pub const HELP: &str = "The Port the MQTT is bound to ";
pub const LONG_HELP: &str = "\
Our Port we bind MQTT to";
pub const LONG: &str = NAME;
pub const NAME: &str = "port";
pub const SHORT: &str = "p";
// for sssl
//pub const DEFAULT_VALUE: &str = "8883";
pub const DEFAULT_VALUE: &str = "1883";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "PORT";

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
