
use clap::Parser;
use kidsnote_sdk::{options::KidsnoteOptions, KidsnoteSdk, auth::error_types::AuthError, user::datatypes::MeInfoResponse};

use crate::kidsnote::{KnBackupConfig, KidsnoteConfigProfile};

#[derive(Parser, Debug, Clone)]
pub struct LoginArgs {
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
}

impl LoginArgs {
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

pub struct LoginCommand {
    args: LoginArgs,
    config: KnBackupConfig,
    kidsnote_sdk: KidsnoteSdk
}

impl LoginCommand {

    /// init and run
    pub async fn run(args:&LoginArgs){ 
        let mut args = args.clone();
        //let config_path = args.config_path.clone().unwrap_or_else(|| String::from("~/.knbackup/config.toml"));
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
            args: args.clone(),
            kidsnote_sdk
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
                log::info!("[login] End.");

                self.config.set_default(me.user.username, auth_result.refresh_token.clone());
                self.config.save(self.args.config_path.clone());
            } 
        } else {
            log::error!(target:"login","Error. Invalid args");
        }
    }

    /// 내정보
    async fn step_myinfo(&mut self) -> Result<MeInfoResponse, AuthError>{
        log::info!(target:"myinfo","Start.");
        match self.kidsnote_sdk
            .user()
            .get_myinfo()
            .await
        {
            Ok(result) => {
                log::info!(target:"myinfo","End.");
                Ok(result)
            },
            Err(err) => {
                log::error!(target:"myinfo","Error: {}", err);
                Err(err)
            }
        }
    }

}
