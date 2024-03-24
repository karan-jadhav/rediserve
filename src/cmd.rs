use clap::Parser;

/// Simple program to start a server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Start the server
    #[arg(short, long)]
    pub start: bool,
    /// Path to the .env file
    #[arg(short, long, default_value = ".env")]
    pub env: String,
}
