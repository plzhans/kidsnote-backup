pub mod datatypes;

use std::sync::{Mutex, Arc};
use datatypes::MeInfoResponse;

use crate::{options::KidsnoteOptions, auth::error_types::AuthError};

pub struct KidsnoteUserSdk {
    config: Arc<Mutex<KidsnoteOptions>>,
}

impl KidsnoteUserSdk {
    pub fn new(config:Arc<Mutex<KidsnoteOptions>>) -> KidsnoteUserSdk {
        Self { 
            config
        }
    }

    pub async fn get_myinfo(&self) -> Result<MeInfoResponse, AuthError> {
        let config = self.config.lock().unwrap();
        let session = config.get_default_session_or_error()?;

        let url = format!("{}/v1/me/info/", config.get_host_ref());

        let client = reqwest::Client::new();
        let response = client.get(url)
            .header("Content-Type", "application/json")
            //.header("User-Agent", "kidsnote/4.41.1 (Build/11382) (iPhone; iOS 16.2; Scale/3.00)")
            .header("Authorization", format!("{} {}", session.token_type, session.access_token))
            //.header("x-device-id", "")
            .send()
            .await;

        match response {
            Ok(response) =>{
                if response.status().is_success()
                {
                    match response.json::<MeInfoResponse>().await {
                        Ok(result) => {
                            Ok(result)
                        },
                        Err(e) => {
                            //log::error!("update_world_multilingual error: {}", e);
                            Err(AuthError::GeneralErrorStr(format!("parse error. {:?}", e)))
                        }
                    }
                } else {
                    //log::error!("update_world_multilingual error: {}", response.status().as_u16());
                    Err(AuthError::GeneralErrorStr(format!("status error. status={:?}", response.status())))
                }
            },
            Err(e) => {
                //log::error!("update_world_multilingual error: {}", e);
                Err(AuthError::GeneralErrorStr(format!("unknown error. {:?}", e)))
            }
        }
    }
}