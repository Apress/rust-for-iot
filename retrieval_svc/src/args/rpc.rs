
use clap::Arg;

pub const HELP: &str = "The RPC Port the App is bound to ";
pub const LONG_HELP: &str = "\
Our RPC Port for our application";
pub const LONG: &str = NAME;
pub const NAME: &str = "rpc";
pub const SHORT: &str = "r";
pub const DEFAULT_VALUE: &str = "5555";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "RPC";

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
