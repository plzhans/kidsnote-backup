use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct KidsnoteAuth {
    pub token_type:String,
    pub access_token:String,
    pub scope:String,
    pub expires_in: i32,
    pub refresh_token:String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthTokenResponse {
    pub token_type:String,
    pub access_token:String,
    pub scope:String,
    pub expires_in: i32,
    pub refresh_token:String,
}