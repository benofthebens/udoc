use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    
    New {
        #[arg(short, long)]
        path: String
    }
}
