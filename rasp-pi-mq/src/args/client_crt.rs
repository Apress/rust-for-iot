use clap::Arg;

pub const HELP: &str = "The Client Certificate";
pub const LONG_HELP: &str = "The Client Cert";
pub const LONG: &str = NAME;
pub const NAME: &str = "client-ca";
pub const SHORT: &str = "c";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "c";
pub const DEFAULT_VALUE: &str = "~/book_certs/PiDevice.pem";

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
