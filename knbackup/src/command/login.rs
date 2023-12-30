use std::path::{PathBuf, Path};

use clap::Parser;
use futures::executor::block_on;
use kidsnote_sdk::{options::KidsnoteOptions, KidsnoteSdk, auth::error_types::AuthError, user::datatypes::MeInfoResponse, child::datatypes::ChildReportResponse};

use crate::command::download::DownloadSource;

#[derive(Parser, Debug)]
pub struct LoginArgs {
    /// Client ID
    #[arg(short = 'c', long = "client_id", env="KNB_CLIENT_ID", value_name = "Client id")]
    pub client_id: Option<String>,

    /// UserID of the Account to greet
    #[arg(short = 'u', long = "user", env="KNB_USER_ID", value_name = "User ID")]
    pub user_id: String,

    /// Password of the Account to greet
    #[arg(short = 'p', long = "pass", env="KNB_USER_PASS", value_name = "User Password")]
    pub user_pass: String,

    // Sets a custom config file
    #[arg(long = "config", value_name = "Config File Path", default_value = "~/.knbackup/config.toml")]
    config: Option<PathBuf>,
}

pub struct LoginCommand {
    kidsnote_sdk: KidsnoteSdk
}

impl LoginCommand {

    /// init and run
    pub fn run(args:&LoginArgs){ 
        let kidsnote_options = KidsnoteOptions::new(args.client_id.clone());
        let kidsnote_sdk = KidsnoteSdk::new(kidsnote_options);

        let mut inst = Self {  
            kidsnote_sdk
        };
        block_on(inst.run_async(args));
    }

    ///
    async fn run_async(&mut self, args:&LoginArgs) {
        println!("[login] Start.");
        match self.kidsnote_sdk
            .auth()
            .login(args.user_id.as_str(), args.user_pass.as_str())
            .await 
        {
            Ok(result) => {
                println!("[login] End.");
                println!("{:?}", result);
                if let Ok(me) = self.step_myinfo().await {
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
            },
            Err(err) => {
                eprintln!("[login] Error: {}", err);
            }
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
