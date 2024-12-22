use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    
    New {
        #[arg(short, long)]
        path: String
    }
}
impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::New { path } => Self::new(path)
        }
    }
    pub fn new(path: &String) {
        println!("Create Directory here {}",*path);
    }
}
