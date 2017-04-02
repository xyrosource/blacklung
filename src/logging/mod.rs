// Copyright 2017 The Xyrosource Team.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate slog;
extern crate slog_term;

use slog::{DrainExt, Logger};

mod version {
    // include the generated version file, containing the
    // VERSION symbol with the version as defined in the
    // cargo metadata file.
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}
use self::version::VERSION;

pub fn setup() -> slog::Logger {
    let drain = slog_term::streamer().compact().build().fuse();
    Logger::root(drain, o!("version" => VERSION))
}
