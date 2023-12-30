mod command;

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
    Download(crate::command::download::DownlaodArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliCommand::Login(args) => LoginCommand::run(args),
        //CliCommand::Auth(args) => AuthCommand::run(args),
        CliCommand::Download(args) => DownloadCommand::run(args)
    }
    
    //println!("User {:?}!", args.user_id);
    //println!("Pass {:?}!", args.user_pass);
}

