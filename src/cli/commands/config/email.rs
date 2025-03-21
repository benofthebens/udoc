use crate::cli::utils::Paths;
use crate::config;
use crate::config::Config;
use std::io;
use std::path::Path;

pub fn set_config_email(email: &String) -> io::Result<()> {
    if !Path::new(&Paths::Data.get()).exists() {
        panic!("This path is not a udoc repository");
    }

    let mut config_json: Config = config::read_config(&Paths::Config.get());

    config_json.user.email = email.to_string();

    config::create_config(&Paths::Config.get(), config_json)?;

    Ok(())
}
