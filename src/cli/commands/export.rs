use std::io::Error;
use std::path::Path;
use crate::cli::utils::Paths;
use crate::log;

pub fn export() -> Result<(), Error> {

    if !Path::new(&Paths::Data.get()).exists() {
        panic!("This is not a udoc repository ");
    }

    let exchange = log::exchange::read_exchange_file(
        &Paths::Exchange.get())?;

    exchange.to_html_file();
    println!("exchange.html file created");
    Ok(())
}