mod command;
mod kidsnote;

use std::env;

use clap::{Parser, Subcommand};
use crate::command::login::LoginCommand;
use crate::command::download::DownloadCommand;

#[derive(Parser, Debug)]
#[command(name = "Kidsnote backup")]
#[command(version = "0.1.0")]
#[command(author = "plzhans <plzhans@gmail.com>")]
#[command(about = "KidsNote backup program", long_about = None)]
#[command(author, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Login(crate::command::login::LoginArgs),
    //Auth(crate::command::auth::AuthArgs),
    Download(crate::command::download::DownloadArgs),
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        CliCommand::Login(args) => LoginCommand::run(args).await,
        //CliCommand::Auth(args) => AuthCommand::run(args),
        CliCommand::Download(args) => DownloadCommand::run(args).await
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