use clap::{CommandFactory, Parser};
use rediserve::cmd::Args;
use rediserve::web::start_server;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.start {
        start_server(args).await;
    } else {
        Args::command().print_help().unwrap();
        std::process::exit(1);
    }
}
