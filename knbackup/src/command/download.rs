use chrono::{DateTime, Utc};
use clap::Parser;
use futures::executor::block_on;
use kidsnote_sdk::{resource::datatypes::ResourceImageResponse, KidsnoteSdk, options::KidsnoteOptions, user::datatypes::MeInfoResponse, auth::{error_types::AuthError, datatypes::OAuthTokenResponse}, child::datatypes::ChildReportResponse};

use std::path::Path;

use crate::kidsnote::{KnBackupConfig, KidsnoteConfigProfile};

#[derive(Parser, Debug, Clone)]
pub struct DownloadArgs {
    /// Client ID
    #[arg(short = 'c', long = "client_id", env="KNB_CLIENT_ID", value_name = "Client id")]
    pub client_id: Option<String>,

    /// UserID of the Account to greet
    #[arg(short = 'u', long = "user", env="KNB_USER_ID", value_name = "User ID")]
    pub user_id: Option<String>,

    /// Password of the Account to greet
    #[arg(short = 'p', long = "pass", env="KNB_USER_PASS", value_name = "User Password")]
    pub user_pass: Option<String>,

    /// RefreshToken of the Account to greet
    #[arg(short = 'r', long = "refresh-token")]
    pub refresh_token: Option<String>,

    // Sets a custom config file
    #[arg(long = "config", value_name = "Config File Path", default_value = "~/.knbackup/config.toml")]
    pub config_path: String,

    /// Backup start date
    #[arg(long, alias = "ds", value_name = "Start Date")]
    pub date_start: Option<String>,

    /// Backup end date
    #[arg(long, alias = "de", value_name = "End Date")]
    pub date_end: Option<String>,
}

impl DownloadArgs { 
    /// 
    fn update_profile(&mut self, profile:&KidsnoteConfigProfile){
        if self.refresh_token.is_none() &&  profile.refresh_token.is_some() {
            self.refresh_token = profile.refresh_token.clone();
        }
        if self.user_id.is_none() &&  profile.user_id.is_some() {
            self.user_id = profile.user_id.clone();
        }
    }
}

pub struct DownloadCommand {
    config: KnBackupConfig,
    kidsnote_sdk: KidsnoteSdk
}

impl DownloadCommand {
    pub fn run(args:&DownloadArgs) { 
        let mut args = args.clone();
        let config_path = args.config_path.clone();
        let config = KnBackupConfig::from_file(&config_path);
        if let Some(profile) = &config.default {
            args.update_profile(profile);
        }

        let kidsnote_options = KidsnoteOptions::new(args.client_id.clone());
        let kidsnote_sdk = KidsnoteSdk::new(kidsnote_options);
        if let Some(refresh_token) = args.refresh_token.clone() {
            kidsnote_sdk.set_refresh_token(refresh_token, args.user_id.clone());
        }

        let mut inst = Self {
            config,
            kidsnote_sdk
        };
        block_on(inst.run_async(args));
    }

    /// 
    async fn run_async(&mut self, args:DownloadArgs) {
        println!("[login] Start.");

        let auth_result = if let Some(refresh_token) = args.refresh_token.clone() {
            println!("[login][refresh_token_login] Start.");
            match self.kidsnote_sdk
                .auth()
                .refresh_token(refresh_token.as_str())
                .await 
            {
                Ok(result) => {
                    println!("[login][refresh_token_login] End.");
                    Some(result)
                },
                Err(err) => {
                    println!("[login][refresh_token_login] Error. {}", err);
                    None
                }
            }
        } else if let (Some(user_id), Some(user_pass)) = (args.user_id.clone(), args.user_pass.clone())  {
            println!("[login][password_login] Start.");
            match self.kidsnote_sdk
                .auth()
                .login(user_id.as_str(), user_pass.as_str())
                .await
            {
                Ok(result) => {
                    println!("[login][password_login] End.");
                    Some(result)
                },
                Err(err) => {
                    println!("[login][password_login] Error. {}", err);
                    None
                }
            }
        } else {
            None
        };

        if let Some(auth_result) = auth_result {
            println!("[login] End.");
            println!("{:?}", auth_result);

            if let Ok(me) = self.step_myinfo().await {
                self.config.set_default(me.user.username, auth_result.refresh_token.clone());
                self.config.save(args.config_path.clone());

                for child in me.children {
                    let mut download_sources = Vec::new();
                    if let Ok(child_report) = self.step_child_report(child.id).await {
                        for report in child_report.results {
                            download_sources.push(DownloadSource {
                                report_id: report.id,
                                report_date: report.created,
                                child_id: child.id,
                                child_name: child.name.clone(),
                                attached_images: report.attached_images
                            });
                        }
                    }  
                    self.step_child_report_download(download_sources).await;
                }
            }   
        } else {
            eprintln!("[login] Error. Invalid args");
        }
    }

    /// 내정보
    async fn step_myinfo(&mut self) -> Result<MeInfoResponse, AuthError>{
        println!("[myinfo] Start.");
        match self.kidsnote_sdk
            .user()
            .get_myinfo()
            .await
        {
            Ok(result) => {
                println!("[myinfo] End.");
                println!("{:?}", result);
                Ok(result)
            },
            Err(err) => {
                eprintln!("[myinfo] Error: {}", err);
                Err(err)
            }
        }
    }

    /// 알림장
    async fn step_child_report(&mut self, child_id:u64) -> Result<ChildReportResponse, AuthError>{
        println!("[child][{}][report] Start", child_id);
        match self.kidsnote_sdk
            .child()
            .get_reports(child_id)
            .await
        {
            Ok(result) => {
                println!("[child][{}][report] End.", child_id);
                println!("{:?}", result);
                Ok(result)
            },
            Err(err) => {
                eprintln!("[child][{}][report] Error: {}", child_id, err);
                Err(err)
            }
        }
    }

    async fn step_child_report_download(&mut self, sources:Vec<DownloadSource>) {
        for source in sources {
            println!("[child][{}][report][download] Start", source.child_id);

            // 이미지 다운로드 받기
            // - /{child_name}/{yyyy-MM-dd}/report_{yyyyMMdd}_{report_id}_{image_id}.png
            for image in source.attached_images {
                //tokio::time::sleep(Duration::from_millis(100)).await;
                let original_file = Path::new(&image.original_file_name);
                let extension = original_file.extension().and_then(|e| e.to_str()).unwrap_or_else(|| "png");
                let path = format!("/{}/{}/report_{}_{}_{}.{}", source.child_name, source.report_date.format("%Y-%m-%d"), source.report_id, source.report_date.format("%Y%m%d"), image.id, extension);
                println!("[image][{}] Download. url={} => {}", image.id, image.original, path);
            }

            // (미구현) 비디오 다운로드 받기
            // (미구현) 첨부파일 다운로드 받기
            // (미구현) 알림장 텍스트 다운로드 받기
            // (미구현) 댓글 다운로드 받기

            println!("[child][{}][report][download] End", source.child_id);
        }
    }
}

pub struct DownloadSource {
    pub report_id: u64,
    pub report_date: DateTime<Utc>,
    pub child_id: u64,
    pub child_name: String,
    pub attached_images: Vec<ResourceImageResponse>,
}