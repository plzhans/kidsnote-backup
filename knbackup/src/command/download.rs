use chrono::{DateTime, Utc};
use clap::Parser;
use filetime::FileTime;
use kidsnote_sdk::{resource::datatypes::ResourceImageResponse, KidsnoteSdk, options::KidsnoteOptions, user::datatypes::MeInfoResponse, auth::error_types::AuthError, child::datatypes::GetReportsParam, tool::image_tool::ImageTool};

use std::{path::{Path, PathBuf}, time::Duration, collections::HashMap};

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

    #[arg(short = 't', long = "test")]
    pub test: bool,
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
            output_dir: "./output".to_string(),
            test: false,
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

pub struct DownloadSource {
    pub report_id: u64,
    pub report_date: DateTime<Utc>,
    pub report_content: Option<String>,
    pub author_name: String,
    pub center_id: u64,
    pub center_name: String,
    pub class_id: u64,
    pub class_name: String,
    pub child_id: u64,
    pub child_name: String,
    pub attached_images: Vec<ResourceImageResponse>,
}

pub struct DownloadCommand {
    args: DownloadArgs,
    config: KnBackupConfig,
    kidsnote_sdk: KidsnoteSdk,
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
            args,
            config,
            kidsnote_sdk,
        };
        inst.next().await;
    }

    /// 
    async fn next(&mut self) {
        log::info!(target:"login","kidsnote user refresh_token checking..");
        let auth_result = if let Some(refresh_token) = self.args.refresh_token.clone() {
            log::info!(target:"login","kidsnote user refresh_token login mode start.");
            match self.kidsnote_sdk
                .auth()
                .refresh_token(refresh_token.as_str())
                .await 
            {
                Ok(result) => {
                    log::info!(target:"login","kidsnote user refresh_token succeeded.");
                    Some(result)
                },
                Err(err) => {
                    log::error!(target:"login","kidsnote user refresh_token fail. {}", err);
                    None
                }
            }
        } else if let (Some(user_id), Some(user_pass)) = (self.args.user_id.clone(), self.args.user_pass.clone())  {
            log::info!(target:"login","kidsnote user password login mode start.");
            match self.kidsnote_sdk
                .auth()
                .login(user_id.as_str(), user_pass.as_str())
                .await
            {
                Ok(result) => {
                    log::info!(target:"login","kidsnote user password login succeeded.");
                    Some(result)
                },
                Err(err) => {
                    log::error!(target:"login","kidsnote user password login fail. {}", err);
                    None
                }
            }
        } else {
            None
        };

        if let Some(auth_result) = auth_result {
            if let Ok(me) = self.step_myinfo().await {
                self.config.set_default(me.user.username, auth_result.refresh_token.clone());
                self.config.save(self.args.config_path.clone());

                for child in me.children {
                    log::info!(target:"myinfo", "[child][{}] center look up.", child.name);
                    let mut center_map = HashMap::new();
                    for enroll in child.enrollment {
                        if !center_map.contains_key(&enroll.center_id) {
                            center_map.insert(enroll.center_id, enroll.center_name);
                        }
                    }

                    if let Err(_err) = self.step_child_report_download(
                        child.id, 
                        child.name.clone(),
                        center_map
                    ).await 
                    {
                        
                    }
                }
            }   
        } else {
            log::error!(target:"login","Error. Invalid args");
        }
    }

    /// 내정보
    async fn step_myinfo(&mut self) -> Result<MeInfoResponse, AuthError>{
        log::info!(target:"myinfo","kidsnote user info look up start.");
        match self.kidsnote_sdk
            .user()
            .get_myinfo()
            .await
        {
            Ok(result) => {
                log::info!(target:"myinfo","kidsnote user info look up succeeded. username={}", result.user.username);
                log::info!(target:"myinfo", "child look up");
                for child in &result.children {
                    if child.enrollment.len() == 0 {
                        log::info!(target:"myinfo","child. name={}", child.name);
                    } else {
                        for enroll in &child.enrollment {
                            log::info!(target:"myinfo","name={}, enrollment={} - {}", child.name, enroll.center_name, enroll.class_name);
                        }
                    }
                }
                Ok(result)
            },
            Err(err) => {
                log::error!(target:"myinfo","kidsnote user info look up fail.");
                log::error!("{}", err);
                Err(err)
            }
        }
    }

    /// 알림장
    async fn step_child_report_download(&mut self, child_id:u64, child_name:String, center_map:HashMap<u64, String>) -> Result<i32, AuthError>{
        log::info!(target:"report","Child:[{}]:Start", child_id);

        let mut result = 0;

        // cls로는 필터링 되는데 center로는 필터가 안된다. 
        let mut report_options = GetReportsParam::new();

        let mut loop_count = 0;
        loop {
            loop_count = loop_count+1;

            log::info!(target: "report", "[Child][{}][report][{:?}] look up. page={}", child_name, report_options.page, child_name);
            match self.kidsnote_sdk
                .child()
                .get_reports(child_id, Some(report_options.clone()))
                .await
            {
                Ok(report_result) => {
                    report_options.page = report_result.next.clone();
                    
                    if report_result.results.len() > 0 {
                        let mut download_sources = Vec::new();
                        for report in report_result.results {
                            let center_name = center_map.get(&report.center).map(|f| f.clone()).unwrap_or_else(|| report.center.to_string());
                            download_sources.push(DownloadSource {
                                report_id: report.id,
                                report_date: report.created,
                                report_content: report.content,
                                author_name: report.author_name,
                                center_id: report.center,
                                center_name,
                                class_id: report.cls,
                                class_name: report.class_name.clone(),
                                child_id,
                                child_name: child_name.clone(),
                                attached_images: report.attached_images
                            });
                            result = result + 1;
                        }
                        self.step_child_report_sourece_download(download_sources).await;
                    } else {
                        report_options.page = None;
                    }
                },
                Err(err) => {
                    report_options.page = None;
                    log::error!(target: "report", "[Child][{}][report][{:?}] look up error. {}", child_name, report_options.page, err);
                    return Err(err);
                }
            }

            if report_options.page.is_none() {
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        log::info!("[child][{}][report] End.", child_id);

        Ok(result)
    }

    async fn step_child_report_sourece_download(&mut self, sources:Vec<DownloadSource>) {
        for source in sources {
            // 알림장 텍스트 변환해서 저장
            let file_time = FileTime::from_unix_time(source.report_date.timestamp(), 0);
            let title = format!("{} {} 알림장", source.report_date.format("%Y-%m-%d"), source.center_name);

            let mut output_base_path = PathBuf::from(&self.args.output_dir);
            output_base_path.push(format!("[{}] {} {}", source.report_date.format("%Y").to_string(), source.center_name, source.child_name));
            output_base_path.push(format!("[{}] {} {}", source.report_date.format("%Y-%m").to_string(), source.center_name, source.child_name));

            let mut output_file = output_base_path.clone();
            output_file.push(format!("{}_알림장_{}_{}_{}.jpg",source.center_name, source.report_date.format("%Y%m%d"), source.child_name, source.report_id));

            if let Some(contents) = source.report_content {
                if !contents.trim().is_empty() {
                    let report_contents: Vec<&str> = contents.split('\n')
                        .map(|s| s.trim())
                        .collect();
                    if report_contents.len() > 0 {
                        if let Some(output_file ) = output_file.to_str(){
                            if !self.args.test {
                                
                                match ImageTool::text_to_image(title.as_str(), &source.author_name, &report_contents, output_file, file_time) {
                                    Ok(_) => {
                                        log::info!(target: "report", "[Child][{}][report][{}][Content] Convert text to image and save.", source.child_name, source.report_id);
                                    },
                                    Err(err) => {
                                        log::error!(target: "report", "[Child][{}][report][{}][Content] Convert text to image and save error. {}", source.child_name, source.report_id, err);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 이미지 다운로드 받기
            // - /{child_name}/{yyyy-MM-dd}/report_{yyyyMMdd}_{center_name}_{report_id}_{image_id}.png
            for image in source.attached_images {
                let original_file = Path::new(&image.original_file_name);
                let extension = original_file.extension().and_then(|e| e.to_str()).unwrap_or_else(|| "png");
                let mut output_file = output_base_path.clone();
                output_file.push(format!("{}_알림장_{}_{}_{}_{}.{}",source.center_name, source.report_date.format("%Y%m%d"), source.child_name, source.report_id, image.id, extension));
                //let path = format!("{}/{}/{}/report_{}_{}_{}_{}.{}", self.args.output_dir, source.child_name, source.report_date.format("%Y-%m-%d"), source.center_name, source.report_id, source.report_date.format("%Y%m%d"), image.id, extension);
                if let Some(output_file) = output_file.to_str() {
                    if !self.args.test {
                        match self.kidsnote_sdk.resource()
                            .download_image(&image.original, image.file_size, file_time, output_file)
                            .await 
                        {
                            Ok(download_result) =>{
                                if download_result {
                                    log::info!(target: "report", "[Child][{}][report][{}][Image][{}] download.", source.child_name, source.report_id, image.id);
                                    log::debug!(target: "report", "File created. path={}", output_file);
                                } else {
                                    log::info!(target: "report", "[Child][{}][report][{}][Image][{:?}] download skip.", source.child_name, source.report_id, image.id);
                                    log::debug!(target: "report", "File skip. path={}", output_file);
                                }
                            },
                            Err(err) => {
                                log::error!(target: "report", "[Child][{}][report][{}][Image][{}] download error. {}", source.child_name, source.report_id, image.id, err);
                            }
                        }
                    } else {
                        log::info!(target: "report", "[Test][Child][{}][report][{}][Image][{}] download.", source.child_name, source.report_id, image.id);
                        log::error!(target: "report", "Download error. path={}", output_file);

                    }
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            }

            // (미구현) 비디오 다운로드 받기
            // (미구현) 첨부파일 다운로드 받기
            // (미구현) 알림장 텍스트 다운로드 받기
            // (미구현) 댓글 다운로드 받기
        }
    }
}
