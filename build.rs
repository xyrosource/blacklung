// Copyright 2017 The Xyrosource Team.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This build.rs script is inspired from
// https://github.com/simias/pockystation/blob/master/build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let version_file = Path::new(&out_dir).join("version.rs");
    let mut fil = File::create(&version_file).unwrap();

    let cargo_version = env!("CARGO_PKG_VERSION").to_owned();

    writeln!(fil, "pub const VERSION: &'static str = \
                 \"{}\";", cargo_version).unwrap();
}

