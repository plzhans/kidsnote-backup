use serde::{Deserialize, Serialize};

use crate::auth::{datatypes::{OAuthTokenResponse, KidsnoteAccessToken}, error_types::{AuthError, AuthErrorCode}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KidsnoteOptions {
    host: String,
    client_id: String,
    user_id: Option<String>,
    refresh_token: Option<String>,
    access_token: Option<KidsnoteAccessToken>
}

impl KidsnoteOptions {
    pub fn new(client_id:Option<String>) -> KidsnoteOptions {
        Self { 
            host: String::from("https://kapi.kidsnote.com"),
            client_id: client_id.unwrap_or_else(|| 
                String::from("eTU0bU4xbHBhWTcyTmlQTEZPQnp5WlNkS2FMV0h4ZUNUV0VoUXp4RzpleENKNVQ5TmlzaGc2NkpEQzh1b1NZN29PM1hTVVVVcjlHRG5penVWaGd3TDJyWkpNVkJHY0hYYTh1UDZ2VmlHbGE2VERGVE8ybDFIMEw3cEdIckFRQ1lpMWsyakEwVTVVT2RmQ2pXeDdSVVJDMk0xZlhhd1ZNRXBIdGJDZExQbQ==")
            ),
            user_id: None,
            refresh_token: None,
            access_token: None
        }
    }

    pub fn set_client_id(&mut self, client_id:String) {
        self.client_id = client_id;
    }

    pub fn set_host(&mut self, host:String) {
        self.host = host;
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.clone()
    }

    pub fn get_client_id_ref(&self) -> &str {
        self.client_id.as_str()
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_host_ref(&self) -> &str {
        self.host.as_str()
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        self.refresh_token.clone()
    }

    pub fn get_access_token_or_error(&self) -> Result<KidsnoteAccessToken, AuthError> {
        if let Some(access_token) = &self.access_token {
            Ok(access_token.clone())
        } else {
            Err(AuthError::ErrorWithCode(AuthErrorCode::Unauthorized))
        }
    }

    pub fn set_user_id(&mut self, user_id:String) {
        self.user_id = Some(user_id);
    }

    pub fn set_refresh_token(&mut self, refresh_token:String) {
        self.refresh_token = Some(refresh_token);
    }

    pub fn set_session_by_oauth(&mut self, data:OAuthTokenResponse) {
        self.refresh_token = Some(data.refresh_token);
        self.access_token = Some(KidsnoteAccessToken {
            r#type: data.token_type,
            token: data.access_token,
            expires_in: data.expires_in,
        });
    }

    pub fn remove_session(&mut self) {
        self.refresh_token = None;
        self.access_token = None;
    }

    pub fn is_refresh_token(&self) -> bool { 
        self.refresh_token.is_some()
    }

}