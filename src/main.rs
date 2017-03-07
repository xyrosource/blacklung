#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate blacklung;

use blacklung::server;

fn main() {
    use blacklung::server::console::Console;
    for cmd in Console::new() {
        match cmd {
            server::console::Command::Dummy => println!("Dummy command received"),
        }
    }
//    if let Err(ref e) = server::start() {
//        use std::io::Write;
//        let stderr = &mut ::std::io::stderr();
//        let errmsg = "Error writing to stderr";
//
//        writeln!(stderr, "error: {}", e).expect(errmsg);
//
//        for e in e.iter().skip(1) {
//            writeln!(stderr, "caused by: {}", e).expect(errmsg);
//        }
//
//        if let Some(backtrace) = e.backtrace() {
//            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
//        }
//
//        ::std::process::exit(1);
//    }
}
