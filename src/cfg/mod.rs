use docopt::Docopt;
use slog::Logger;
use std::fs::File;
use std::io::Read;
use toml::from_str;

pub mod errors {
    error_chain!{}
}

use self::errors::*;

const CONFIGFILE: &'static str = "blacklung.cfg";

const USAGE: &'static str = "
Blacklung server.

Usage:
    blacklung [--port=<port>]
    blacklung (-h | --help)

Options:
    -h --help       Show this screen.
    --port=<PORT>   Port to bind to. Needs to be detailed here or in configuration file.
";

/// This struct is used for the argument parsing.
#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: Option<u16>,
}

/// This struct holds data that can be read from the config file;
/// hence the Option.
#[derive(Debug, Deserialize)]
struct ConfigurationFile {
    pub port: Option<u16>,
}

impl ConfigurationFile {
    pub fn new() -> ConfigurationFile {
        ConfigurationFile {
            port: None,
        }
    }
}


/// This struct holds the actual configuration data
#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
}

fn read_config() -> Result<ConfigurationFile> {
    // the configuration file is optional, so we'll return a
    // default constructed ConfigurationFile in that case..
    match File::open(CONFIGFILE) {
        Ok(mut file) => {
            let mut data = String::new();
            // .. but if it does exist, then reading should succeed..
            file.read_to_string(&mut data).
                chain_err(|| "Failed to read from file")?;

            // .. and the parsing should succeed..
            let cfg: ConfigurationFile = from_str(data.as_str()).
                chain_err(|| "Failed to parse configuration file")?;

            Ok(cfg)
        },
        Err(_) => Ok(ConfigurationFile::new())
    }
}

fn join(cfg: ConfigurationFile, args: Args) -> Result<Config> {
    let _port = match cfg.port.or(args.flag_port) {
        Some(v) => v,
        None    => bail!("Port not configured or given as command line argument."),
    };

    Ok(Config {
        port: _port,
    })
}

pub fn get_config(root_logger: &Logger) -> Result<Config> {
    let cfg = read_config()
        .chain_err(|| "Failed to read the configuration file.")?;

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .chain_err(|| "Failed to parse command line arguments")?;

    info!(root_logger, "Application configuration read";
          "cfg" => format!("{:?}", cfg));

    join(cfg, args)
}
