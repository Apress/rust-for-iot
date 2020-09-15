
use clap::Arg;

pub const HELP: &str = "The server address for the RPC to bind to";
pub const LONG_HELP: &str = "\
The address the server will attempt to bind to for the RPC server";
pub const LONG: &str = NAME;
pub const NAME: &str = "rpc_server";
pub const SHORT: &str = "y";
pub const DEFAULT_VALUE: &str = "127.0.0.1";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "RPC_SERVER_ADDR";

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
