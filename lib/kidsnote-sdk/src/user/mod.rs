pub mod datatypes;

use std::sync::{Mutex, Arc};
use datatypes::MeInfoResponse;

use crate::{options::KidsnoteOptions, auth::error_types::AuthError};

pub struct KidsnoteUserSdk {
    options: Arc<Mutex<KidsnoteOptions>>,
}

impl KidsnoteUserSdk {
    pub fn new(options:Arc<Mutex<KidsnoteOptions>>) -> KidsnoteUserSdk {
        Self { 
            options
        }
    }

    pub async fn get_myinfo(&self) -> Result<MeInfoResponse, AuthError> {
        let options = self.options.lock().unwrap();
        let access_token = options.get_access_token_or_error()?;

        let url = format!("{}/v1/me/info/", options.get_host_ref());

        let client = reqwest::Client::new();
        let response = client.get(url)
            .header("Content-Type", "application/json")
            //.header("User-Agent", "kidsnote/4.41.1 (Build/11382) (iPhone; iOS 16.2; Scale/3.00)")
            .header("Authorization", format!("{} {}", access_token.r#type, access_token.token))
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