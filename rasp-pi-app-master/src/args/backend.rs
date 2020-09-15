use clap::Arg;

pub const HELP: &str = "The upload_svc endpoint full path";
pub const LONG_HELP: &str = "\
This will be in the format http://localhost:8080 for our endpoint to communicate to";
pub const LONG: &str = NAME;
pub const NAME: &str = "backend";
pub const SHORT: &str = "b";
pub const DEFAULT_VALUE: &str = "http://localhost:8080";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "BACKEND";

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
