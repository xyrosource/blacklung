#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate slog;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod server;
pub mod logging;
pub mod cfg;
