
use clap::Arg;

pub const HELP: &str = "A Database URL For our PostGres DtABASE ";
pub const LONG_HELP: &str = "\
This is the database URL to be used for our application, this is the postgres\
database url. This url should be in the format: \
DATABASE_URL=postgres://user:password@localhost:5432/diesel_db_iron_example";
pub const LONG: &str = NAME;
pub const NAME: &str = "database-url";
pub const SHORT: &str = "d";
pub const DEFAULT_VALUE: &str = "postgres://user:password@localhost:5433/rust-iot-db";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "DATABASE_URL";

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
