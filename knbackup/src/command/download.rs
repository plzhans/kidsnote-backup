use chrono::{DateTime, Utc};
use clap::Parser;
use kidsnote_sdk::resource::datatypes::ResourceImageResponse;


use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct DownlaodArgs {
    /// UserID of the Account to greet
    #[arg(short = 'u', long, env="KNB_USER_ID", value_name = "User ID")]
    pub user_id: Option<String>,

    /// Password of the Account to greet
    #[arg(long, short = 'p', env="KNB_USER_PASS", value_name = "User Password")]
    pub user_pass: Option<String>,

    /// Backup start date
    #[arg(long, alias = "ds", value_name = "Start Date")]
    pub date_start: Option<String>,

    /// Backup end date
    #[arg(long, alias = "de", value_name = "End Date")]
    pub date_end: Option<String>,

    // // Sets a custom config file
    // #[arg(short, long, value_name = "Config File Path", default_value = "./config.conf")]
    // config: Option<PathBuf>,

}



pub struct DownloadCommand {}

impl DownloadCommand {
    pub fn run(args:&DownlaodArgs) { 
        let inst = Self {  };
        inst.internal_run(args);
    }

    fn internal_run(&self, args:&DownlaodArgs) {
    }
}

pub struct DownloadSource {
    pub report_id: u64,
    pub report_date: DateTime<Utc>,
    pub child_id: u64,
    pub child_name: String,
    pub attached_images: Vec<ResourceImageResponse>,
}