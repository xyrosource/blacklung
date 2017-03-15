#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate blacklung;
#[macro_use]
extern crate slog;
extern crate toml;

use blacklung::server;
use blacklung::logging;
use blacklung::cfg;
use std::process;


/// Execute the closure, and return its result if Ok. If Err,
/// the the error message and chain will be printed, and the
/// application will exit with error.
fn exit_on_error<T, V, E>(f: T) -> V
    where T: Fn() -> Result<V, E>,
          E: error_chain::ChainedError
{
    match f() {
        Ok(v) => v,
        Err(ref e) => {
            use std::io::Write;
            let stderr = &mut ::std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);

            for e in e.iter().skip(1) {
                writeln!(stderr, "caused by: {}", e).expect(errmsg);
            }
            process::exit(1)
        },
    }
}

fn main() {
    let root_logger = logging::setup();
    let config = exit_on_error(|| { cfg::get_config(&root_logger) });

    info!(root_logger, "Started application"; "args" => format!("{:?}", config));

    exit_on_error(|| { server::start(&root_logger, config.port) });
}
