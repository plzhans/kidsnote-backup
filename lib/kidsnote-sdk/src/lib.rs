pub mod options;
pub mod resource;
pub mod auth;
pub mod user;
pub mod child;

use std::sync::{Arc, Mutex};
use child::KidsnoteChildSdk;
use options::KidsnoteOptions;
use auth::KidsnoteAuthSdk;
use user::KidsnoteUserSdk;

pub struct KidsnoteSdk {
    config: Arc<Mutex<KidsnoteOptions>>,
    auth: KidsnoteAuthSdk,
    user: KidsnoteUserSdk,
    child: KidsnoteChildSdk,
}

impl KidsnoteSdk {
    pub fn new(config:KidsnoteOptions) -> KidsnoteSdk {
        let config_arc = Arc::new(Mutex::new(config));
        let auth = KidsnoteAuthSdk::new(Arc::clone(&config_arc));
        let user: KidsnoteUserSdk = KidsnoteUserSdk::new(Arc::clone(&config_arc));
        let child = KidsnoteChildSdk::new(Arc::clone(&config_arc));
        KidsnoteSdk { 
            config: config_arc,
            auth,
            user,
            child
        }
    }

    /// auth sdk
    pub fn auth(&mut self) -> &mut KidsnoteAuthSdk { &mut self.auth }

    /// child sdk
    pub fn child(&mut self) -> &mut KidsnoteChildSdk { &mut self.child }

    /// user sdk
    pub fn user(&mut self) -> &mut KidsnoteUserSdk { &mut self.user }

    ///
    pub fn get_client_id(&self) -> String { 
        let config = &self.config.lock().unwrap();
        config.get_client_id()
    }
    
    ///
    pub fn get_host(&self) -> String { 
        let config = &self.config.lock().unwrap();
        config.get_host()
    }
    
}