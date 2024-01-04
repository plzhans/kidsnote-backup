pub mod datatypes;

use std::sync::{Mutex, Arc};
use datatypes::ChildReportResponse;

use crate::{options::KidsnoteOptions, auth::error_types::AuthError};

use self::datatypes::GetReportsParam;

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
    pub async fn get_reports(&self, child_id:u64, param:Option<GetReportsParam>) -> Result<ChildReportResponse, AuthError> {
        let options = self.options.lock().unwrap();
        let access_token = options.get_access_token_or_error()?;

        let query = serde_urlencoded::to_string(&param).unwrap();
        let url = format!("{}/v1_2/children/{}/reports/?{}", options.get_host_ref(), child_id, query);

        let client = crate::common::get_client();
        let response = client.get(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("{} {}", access_token.r#type, access_token.token))
            //.header("User-Agent", "kidsnote/4.41.1 (Build/11382) (iPhone; iOS 16.2; Scale/3.00)")
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
                        Err(err) => {
                            Err(AuthError::GeneralErrorStr(format!("parse error. {:?}", err)))
                        }
                    }
                } else {
                    Err(AuthError::GeneralErrorStr(format!("status error. status={:?}", response.status())))
                }
            },
            Err(err) => {
                Err(AuthError::GeneralErrorStr(format!("unknown error. {:?}", err)))
            }
        }
    }
}