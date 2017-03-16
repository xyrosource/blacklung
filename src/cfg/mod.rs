// Copyright 2017 The Xyrosource Team.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use docopt::Docopt;
use slog::Logger;
use std::fs::File;
use std::io::Read;
use toml::from_str;

pub mod errors {
    error_chain!{}
}

use self::errors::*;


type PortType = Option<u16>;

// NOTE: This is detailed in USAGE as well, so if this one is changed,
// make sure to update USAGE as well..
const PORT: u16 = 12345;

const USAGE: &'static str = "
Blacklung server.

Usage:
    blacklung [--port=<port>] [--config=<configfile>]
    blacklung (-h | --help)

Options:
    -h --help               Show this screen.
    --config=<CONFIGFILE>   Configuration file to use. [default: blacklung.cfg].
    --port=<PORT>           Port to bind to. Defaults to 12345, unless given as
                            configuration item or argument.
";

/// Used for the argument parsing.
#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: PortType,
    flag_config: String,
}

/// Details the structure of the configuration file, which will be
/// deseralized into this struct.
#[derive(Debug, Deserialize)]
struct ConfigurationFile {
    pub port: PortType,
}

impl ConfigurationFile {
    pub fn new() -> ConfigurationFile {
        ConfigurationFile { port: None }
    }
}


/// Holds the actual configuration data that is the join of the configured data,
/// the command line options, and any default values.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
}

/// Attempt to deserialize configfile into a ConfigurationFile. If the
/// configuration file does not exist, a default constructed ConfigurationFile
/// will be returned; this function will Err if the configuration file exists
/// but fails to read or parse.
fn read_config(configfile: &str) -> Result<ConfigurationFile> {
    // the configuration file is optional, so we'll return a
    // default constructed ConfigurationFile in that case..
    match File::open(configfile) {
        Ok(mut file) => {
            let mut data = String::new();
            // .. but if it does exist, then reading should succeed..
            file.read_to_string(&mut data)
                .chain_err(|| "Failed to read from file")?;

            // .. and the parsing should succeed..
            let cfg: ConfigurationFile =
                from_str(&data).chain_err(|| "Failed to parse configuration file")?;

            Ok(cfg)
        }
        Err(_) => Ok(ConfigurationFile::new()),
    }
}

/// Join any configured values, command line argument values and default values
/// into a Config struct that details the complete configuration.
fn join(cfg: ConfigurationFile, args: Args) -> Result<Config> {
    let _port = match args.flag_port.or(cfg.port) {
        Some(v) => v,
        None => PORT,
    };

    Ok(Config { port: _port })
}

/// Create the complete Config based on configuration values, command line
/// arguments and default values. This function will Err upon malformed
/// configuration items or agruments.
pub fn get_config(root_logger: &Logger) -> Result<Config> {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
        .chain_err(|| "Failed to parse command line arguments")?;

    let cfg =
        read_config(&args.flag_config).chain_err(|| "Failed to read the configuration file.")?;

    info!(root_logger, "Application configuration read";
          "cfg" => format!("{:?}", cfg));

    join(cfg, args)
}
