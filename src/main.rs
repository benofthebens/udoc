mod cmd;
mod config; 
mod log; 

use clap::Parser;
use crate::cmd::Cmd;

fn main() {
    let path: String = "err".to_string();

    log::create_log_file(&path);
}
