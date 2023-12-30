use crate::auth::{datatypes::{KidsnoteAuth, OAuthTokenResponse}, error_types::{AuthError, AuthErrorCode}};

pub struct KidsnoteOptions {
    host: String,
    client_id: String,
    auth: Option<KidsnoteAuth>
}

impl KidsnoteOptions {
    pub fn new(client_id:Option<String>) -> KidsnoteOptions {
        Self { 
            host: String::from("https://kapi.kidsnote.com"),
            client_id: client_id.unwrap_or_else(|| 
                String::from("eTU0bU4xbHBhWTcyTmlQTEZPQnp5WlNkS2FMV0h4ZUNUV0VoUXp4RzpleENKNVQ5TmlzaGc2NkpEQzh1b1NZN29PM1hTVVVVcjlHRG5penVWaGd3TDJyWkpNVkJHY0hYYTh1UDZ2VmlHbGE2VERGVE8ybDFIMEw3cEdIckFRQ1lpMWsyakEwVTVVT2RmQ2pXeDdSVVJDMk0xZlhhd1ZNRXBIdGJDZExQbQ==")
            ),
            auth: None
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

    pub fn get_default_session_or_error(&self) -> Result<KidsnoteAuth, AuthError> {
        if let Some(auth) = &self.auth {
            Ok(auth.clone())
        } else {
            Err(AuthError::ErrorWithCode(AuthErrorCode::Unauthorized))
        }
    }

    pub fn set_default_session(&mut self, data:OAuthTokenResponse) {
        self.auth = Some(KidsnoteAuth {
            token_type: data.token_type,
            access_token: data.access_token,
            scope: data.scope,
            expires_in: data.expires_in,
            refresh_token: data.refresh_token,
        });
    }

    pub fn remove_default_session(&mut self) {
        self.auth = None;
    }
}