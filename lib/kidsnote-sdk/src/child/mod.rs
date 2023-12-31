pub mod datatypes;

use std::sync::{Mutex, Arc};
use datatypes::ChildReportResponse;

use crate::{options::KidsnoteOptions, auth::error_types::AuthError};

pub struct KidsnoteChildSdk {
    options: Arc<Mutex<KidsnoteOptions>>,
}

impl KidsnoteChildSdk {
    pub fn new(config:Arc<Mutex<KidsnoteOptions>>) -> KidsnoteChildSdk {
        Self { 
            options: config
        }
    }

    /// 알림장 조회
    pub async fn get_reports(&self, child_id:u64) -> Result<ChildReportResponse, AuthError> {
        let options = self.options.lock().unwrap();
        let access_token = options.get_access_token_or_error()?;

        let url = format!("{}/v1_2/children/{}/reports/", options.get_host_ref(), child_id);

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
                    match response.json::<ChildReportResponse>().await {
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