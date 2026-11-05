use std::{collections::HashMap, error::Error, fs, num::ParseIntError, str::FromStr};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub path: String,
}

pub struct Config {
    pub server_name: String,
    pub port: u16,
    pub max_connections: u32,
    pub debug: bool,
}

impl FromStr for Config {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|&line| !line.is_empty()).collect();
        let server_name: String;
        let port: u16;
        let max_connections: u32;
        let debug;
        let mut mapped_config = HashMap::new();

        for line in lines {
            let split = line.split_once("=");
            if let Some(touple) = split {
                mapped_config.insert(touple.0, touple.1);
            } else {
                panic!("Bad config format.")
            }
        }

        server_name = match mapped_config.get("server_name") {
            Some(name) => String::from(*name),
            None => panic!("No server_name provided."),
        };

        port = match mapped_config.get("port") {
            Some(port_parse) => u16::from_str_radix(*port_parse, 10)?,
            None => panic!("No port provided."),
        };

        max_connections = match mapped_config.get("max_connections") {
            Some(connections) => u32::from_str_radix(*connections, 10)?,
            None => panic!("No max_connections provided."),
        };

        debug = match mapped_config.get("debug") {
            Some(value) => {
                if String::from(*value) == String::from("true") {
                    true
                } else {
                    false
                }
            }
            None => panic!("No debug was provided."),
        };

        Ok(Config {
            server_name,
            port,
            max_connections,
            debug,
        })
    }
}

impl Config {
    pub fn read(path: String) -> Result<Config, Box<dyn Error>> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(Config::from_str(&content).unwrap()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CONFIG: &str = "\
server_name=localhost
port=8080
max_connections=100
debug=true";

    #[test]
    fn parses_valid_config() {
        let config = Config::from_str(VALID_CONFIG).unwrap();

        assert_eq!(config.server_name, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_connections, 100);
        assert!(config.debug);
    }

    #[test]
    fn parses_config_with_debug_disabled() {
        let input = "\
server_name=localhost
port=8080
max_connections=100
debug=false";

        let config = Config::from_str(input).unwrap();

        assert!(!config.debug);
    }

    #[test]
    fn ignores_empty_lines() {
        let input = "\
server_name=localhost

port=8080

max_connections=100
debug=true
";

        let config = Config::from_str(input).unwrap();

        assert_eq!(config.server_name, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn rejects_invalid_port() {
        let input = "\
server_name=localhost
port=invalid
max_connections=100
debug=true";

        assert!(Config::from_str(input).is_err());
    }

    #[test]
    fn rejects_invalid_max_connections() {
        let input = "\
server_name=localhost
port=8080
max_connections=invalid
debug=true";

        assert!(Config::from_str(input).is_err());
    }

    #[test]
    #[should_panic(expected = "No server_name provided.")]
    fn rejects_missing_server_name() {
        let input = "\
port=8080
max_connections=100
debug=true";

        let _ = Config::from_str(input);
    }
}
