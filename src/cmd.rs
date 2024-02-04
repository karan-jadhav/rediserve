use clap::Parser;

/// Simple program to start a server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Start the server
    #[arg(short, long)]
    pub start: bool,
}
