#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate blacklung;

use blacklung::server;
use blacklung::logging;

fuzz_target!(|data: &[u8]| {
    let root_logger = logging::setup();
    server::start(&root_logger, 10000);
});
