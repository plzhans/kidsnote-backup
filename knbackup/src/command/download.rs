use chrono::{DateTime, Utc};
use clap::Parser;
use kidsnote_sdk::{resource::datatypes::ResourceImageResponse, KidsnoteSdk, options::KidsnoteOptions, user::datatypes::MeInfoResponse, auth::error_types::AuthError, child::datatypes::GetReportsParam, tool::image_tool::ImageTool};

use std::{path::{Path, PathBuf}, time::Duration};

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

    #[arg(short = 'o', long = "output-path", value_name = "Output Path", default_value = "./output")]
    pub output_dir: String,
}

impl DownloadArgs { 

    pub fn new() -> Self {
        Self { 
            client_id: None,
            user_id: None,
            user_pass: None,
            refresh_token: None,
            config_path: "~/.knbackup/config.toml".to_string(),
            date_start: None,
            date_end: None,
            output_dir: "./output".to_string()
        }
    }

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
    args: DownloadArgs,
    config: KnBackupConfig,
    kidsnote_sdk: KidsnoteSdk
}

impl DownloadCommand {
    pub async fn run(args:&DownloadArgs) { 
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
            args: args,
            config,
            kidsnote_sdk
        };
        inst.run_async().await;
    }

    /// 
    async fn run_async(&mut self) {
        println!("[login] Start.");

        let auth_result = if let Some(refresh_token) = self.args.refresh_token.clone() {
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
        } else if let (Some(user_id), Some(user_pass)) = (self.args.user_id.clone(), self.args.user_pass.clone())  {
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

            if let Ok(me) = self.step_myinfo().await {
                self.config.set_default(me.user.username, auth_result.refresh_token.clone());
                self.config.save(self.args.config_path.clone());

                for child in me.children {
                    for enroll in child.enrollment {
                        if let Ok(_step_child_report_result) = self.step_child_report(
                            child.id, 
                            child.name.clone(), 
                            enroll.center_id, 
                            enroll.center_name.clone(),
                            enroll.belong_to_class,
                            enroll.class_name.clone()
                        ).await {

                        }
                    }
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
                Ok(result)
            },
            Err(err) => {
                eprintln!("[myinfo] Error: {}", err);
                Err(err)
            }
        }
    }

    /// 알림장
    async fn step_child_report(&mut self, child_id:u64, child_name:String, center_id:u64, center_name:String, class_id:u64, class_name:String) -> Result<i32, AuthError>{
        println!("[child][{}][report] Start", child_id);

        let mut result = 0;

        let mut report_options = GetReportsParam::new();
        report_options.center_id = Some(center_id);
        report_options.cls = Some(class_id);

        let mut loop_count = 0;
        loop {
            loop_count = loop_count+1;

            match self.kidsnote_sdk
                .child()
                .get_reports(child_id, Some(report_options.clone()))
                .await
            {
                Ok(report_result) => {
                    report_options.page = report_result.next.clone();
                    if report_options.page.is_some() {
                        println!("[child][{}][report] Next. loop={}, page={:?}", child_id, loop_count, report_options.page);
                    }

                    let mut download_sources = Vec::new();
                    for report in report_result.results {
                        download_sources.push(DownloadSource {
                            report_id: report.id,
                            report_date: report.created,
                            report_content: report.content,
                            center_id,
                            center_name: center_name.clone(),
                            class_id,
                            class_name: class_name.clone(),
                            child_id,
                            child_name: child_name.clone(),
                            attached_images: report.attached_images
                        });
                        result = result + 1;
                    }
                    self.step_child_report_download(download_sources).await;
                },
                Err(err) => {
                    report_options.page = None;
                    eprintln!("[child][{}][report] Error: loop={}, {}",  child_id, loop_count, err);
                    return Err(err);
                }
            }

            if report_options.page.is_none() {
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        println!("[child][{}][report] End.", child_id);

        Ok(result)
    }

    async fn step_child_report_download(&mut self, sources:Vec<DownloadSource>) {
        for source in sources {
            println!("[child][{}][report][download] Start", source.child_id);

            // 알림장 텍스트 변환해서 저장
            let title = format!("{} {} 알림장", source.report_date.format("%Y-%m-%d"), source.center_name, );

            let mut output_base_path = PathBuf::from(&self.args.output_dir);
            output_base_path.push(source.child_name.clone());
            output_base_path.push(source.center_name.clone());
            output_base_path.push("알림장");
            output_base_path.push(source.report_date.format("%Y").to_string());
            output_base_path.push(source.report_date.format("%Y-%m").to_string());
            output_base_path.push(source.report_date.format("%Y-%m-%d").to_string());

            let mut output_file = output_base_path.clone();
            output_file.push(format!("{}_알림장_{}_{}.jpg",source.center_name, source.report_date.format("%Y%m%d"), source.child_name));

            if let Some(contents) = source.report_content {
                if !contents.trim().is_empty() {
                    let report_contents: Vec<&str> = contents.split('\n')
                        .map(|s| s.trim())
                        .collect();

                    if let Some(output_file ) = output_file.to_str(){
                        ImageTool::text_to_image(title.as_str(), &report_contents, output_file);
                    }
                }
            }
            
            // 이미지 다운로드 받기
            // - /{child_name}/{yyyy-MM-dd}/report_{yyyyMMdd}_{center_name}_{report_id}_{image_id}.png
            for image in source.attached_images {
                let original_file = Path::new(&image.original_file_name);
                let extension = original_file.extension().and_then(|e| e.to_str()).unwrap_or_else(|| "png");
                let mut output_file = output_base_path.clone();
                output_file.push(format!("{}_알림장_{}_{}_{}.{}",source.center_name, source.report_date.format("%Y%m%d"), source.child_name, image.id, extension));
                //let path = format!("{}/{}/{}/report_{}_{}_{}_{}.{}", self.args.output_dir, source.child_name, source.report_date.format("%Y-%m-%d"), source.center_name, source.report_id, source.report_date.format("%Y%m%d"), image.id, extension);
                if let Some(output_file) = output_file.to_str() {
                    match self.kidsnote_sdk.resource()
                        .download_image(&image.original, image.file_size, output_file)
                        .await 
                    {
                        Ok(download_result) =>{
                            println!("[image][{}] Download. url={} => {}", image.id, image.original, output_file);
                            if download_result {
                                tokio::time::sleep(Duration::from_millis(1)).await;
                            } else {
                                println!("[image][{}] Skip. url={} => {}", image.id, image.original, output_file);
                            }
                        },
                        Err(err) => {
                            tokio::time::sleep(Duration::from_millis(1)).await;
                            eprintln!("[image][{}] Download error. url={} => {}, {}", image.id, image.original, output_file, err);
                        }
                    }
                    
                }
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
    pub report_content: Option<String>,
    pub center_id: u64,
    pub center_name: String,
    pub class_id: u64,
    pub class_name: String,
    pub child_id: u64,
    pub child_name: String,
    pub attached_images: Vec<ResourceImageResponse>,
}