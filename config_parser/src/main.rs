use clap::Parser;
use config_parser::{Cli, Config};

fn main() {
    let args = Cli::parse();
    let path = args.path;

    let config = Config::read(path).unwrap();
    println!(
        "Server name: {}\nPort: {}\nConnection limit: {}\nDebug mode: {}",
        config.server_name, config.port, config.max_connections, config.debug
    );
}
