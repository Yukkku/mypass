use mypass::{Config, generate};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ConfigFile {
    services: HashMap<String, Config>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::{arg, command};

    let matches = command!().arg(arg!([service])).get_matches();

    let service = matches
        .get_one::<String>("service")
        .expect("missing service name");

    let xdg_dirs = xdg::BaseDirectories::with_prefix("mypass");
    let config = xdg_dirs.place_config_file("config.toml")?;

    let config = std::fs::read_to_string(config)?;
    let ConfigFile { services } = toml::from_str::<ConfigFile>(&config)?;

    let masterpass = xdg_dirs.place_config_file("masterpass")?;
    let masterpass = std::fs::read(masterpass)?;

    let config = services.get(service).expect("unknown service");

    println!("{}", generate(config, service, &masterpass).as_ref());
    Ok(())
}
