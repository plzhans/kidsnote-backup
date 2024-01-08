mod command;
mod kidsnote;
mod logger;

use std::env;

use crate::command::download::DownloadCommand;
use crate::command::login::LoginCommand;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Kidsnote backup")]
#[command(version = "0.1.0")]
#[command(author = "plzhans <plzhans@gmail.com>")]
#[command(about = "KidsNote backup program", long_about = None)]
#[command(author, long_about = None)]
pub struct Cli {
    #[clap(long, global = true)]
    debug: bool,
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Login(crate::command::login::LoginArgs),
    Download(crate::command::download::DownloadArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.debug {
        env::set_var("RUST_LOG", "debug");
    }
    logger::init();

    match &cli.command {
        CliCommand::Login(args) => LoginCommand::run(args).await,
        //CliCommand::Auth(args) => AuthCommand::run(args),
        CliCommand::Download(args) => DownloadCommand::run(args).await,
    }
}

//#[cfg(tests)]
mod tests {

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn download_test() {
        let args = crate::command::download::DownloadArgs::new();
        crate::command::download::DownloadCommand::run(&args).await;
    }

    // #[ignore]
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // async fn test_method_test() {
    //     crate::test_method();
    // }
}
