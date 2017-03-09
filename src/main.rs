#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate blacklung;
#[macro_use]
extern crate slog;

use blacklung::server;
use blacklung::logging;

fn main() {
    let root_logger = logging::setup();

    info!(root_logger, "Started application");

    if let Err(ref e) = server::start(&root_logger) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
