

use clap::Arg;

pub const HELP: &str = "The Root CA Certificate";
pub const LONG_HELP: &str = "The root CA Cert";
pub const LONG: &str = NAME;
pub const NAME: &str = "root-ca";
pub const SHORT: &str = "r";
pub const TAKES_VALUE: bool = false;
pub const VALUE_NAME: &str = "ROOT_CA";
pub const DEFAULT_VALUE: &str = "~/book_certs/RustIOTRootCA.pem";

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
