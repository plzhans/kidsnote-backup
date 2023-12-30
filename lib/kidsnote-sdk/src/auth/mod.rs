
pub mod datatypes;
pub mod error_types;

use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::options::KidsnoteOptions;

use self::{datatypes::OAuthTokenResponse, error_types::AuthError};

pub struct KidsnoteAuthSdk {
    config: Arc<Mutex<KidsnoteOptions>>,
}

impl KidsnoteAuthSdk {

    pub fn new(config:Arc<Mutex<KidsnoteOptions>>) -> KidsnoteAuthSdk {
        Self { 
            config
        }
    }

    // oauth token
    pub async fn oauth_token(&mut self, params:HashMap<&str, &str>) -> Result<OAuthTokenResponse, AuthError> {

        let mut config = self.config.lock().unwrap();

        let url = format!("{}/o/token/", config.get_host_ref());

        let body = serde_urlencoded::to_string(&params)
            .map_err(|_e| AuthError::GeneralError("serde_urlencoded"))?;

        let client = reqwest::Client::new();
        let response = client.post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            //.header("User-Agent", "kidsnote/4.41.1 (Build/11382) (iPhone; iOS 16.2; Scale/3.00)")
            .header("Authorization", format!("Basic {}", config.get_client_id_ref()))
            .body(body)
            .send()
            .await;

        match response {
            Ok(response) =>{
                if response.status().is_success()
                {
                    match response.json::<OAuthTokenResponse>().await {
                        Ok(result) => {
                            config.set_default_session(result.clone());
                            Ok(result)
                        },
                        Err(e) => {
                            config.remove_default_session();
                            //log::error!("update_world_multilingual error: {}", e);
                            Err(AuthError::GeneralErrorStr(format!("parse error. {:?}", e)))
                        }
                    }
                } else {
                    config.remove_default_session();
                    //log::error!("update_world_multilingual error: {}", response.status().as_u16());
                    Err(AuthError::GeneralErrorStr(format!("status error. status={:?}", response.status())))
                }
            },
            Err(e) => {
                config.remove_default_session();
                //log::error!("update_world_multilingual error: {}", e);
                Err(AuthError::GeneralErrorStr(format!("unknown error. {:?}", e)))
            }
        }
    }

    // login
    pub async fn login(&mut self, username:&str, password:&str) -> Result<OAuthTokenResponse, AuthError> {
        let mut data = HashMap::new();
        data.insert("grant_type", "password");
        data.insert("username", username);
        data.insert("password", password);
        data.insert("scope", "read write");

        self.oauth_token(data).await
    }

    /// refresh token
    pub async fn refresh_token(&mut self, refresh_token:&str) -> Result<OAuthTokenResponse, AuthError> {
        let mut data = HashMap::new();
        data.insert("grant_type", "refresh_token");
        data.insert("refresh_token", refresh_token);
        data.insert("scope", "read write");

        self.oauth_token(data).await
    }
}