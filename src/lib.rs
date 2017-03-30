// Copyright 2017 The Xyrosource Team.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
#[macro_use]
extern crate serde;
extern crate bytes;

pub mod server;
pub mod logging;
pub mod cfg;

mod protocol;
