use crate::cli::utils::Paths;
use crate::config;
use crate::config::Config;
use std::io;
use std::path::Path;

pub fn set_config_name(name: &String) -> io::Result<()> {
    if !Path::new(&Paths::Data.get()).exists() {
        panic!("This path is not a udoc repository");
    }

    let mut config: Config = config::read_config(&Paths::Config.get());
    config.user.username = name.to_string();

    std::fs::remove_file(&Paths::Config.get())?;
    config::create_config(&Paths::Config.get(), config).expect("TODO: panic message");

    Ok(())
}
