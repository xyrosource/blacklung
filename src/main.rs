#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate blacklung;
extern crate docopt;
extern crate rustc_serialize;

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

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if let Err(ref e) = server::start(args.flag_port) {
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
