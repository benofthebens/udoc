mod cmd;

use clap::Parser;
use crate::cmd::Cmd;

fn main() {
    let cmd = Cmd::parse();
    
}
