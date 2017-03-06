extern crate slog;
extern crate slog_term;

use slog::{DrainExt, Logger};

pub fn setup() -> slog::Logger {
    let drain = slog_term::streamer().compact().build().fuse();
    return Logger::root(drain, o!())
}
