#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate blacklung;
extern crate docopt;
extern crate rustc_serialize;
#[macro_use]
extern crate slog;

use blacklung::server;
use docopt::Docopt;

const USAGE: &'static str = "
Blacklung server.

Usage:
    blacklung [--port=<port>]
    blacklung (-h | --help)

Options:
    -h --help       Show this screen.
    --port=<PORT>   Port to bind to [default: 12345].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: u16,
}
use blacklung::logging;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let root_logger = logging::setup();

    info!(root_logger, "Started application"; "args" => format!("{:?}", args));

    if let Err(ref e) = server::start(&root_logger, args.flag_port) {
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
