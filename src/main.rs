use clap::Parser;
use commands::Commands;

mod commands;
mod constants;
mod error;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "Dust-Mail command line interface")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    args.command
        .handle()
        .await
        .expect("Something whent wrong when executing a command");
}
