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
