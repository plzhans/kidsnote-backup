pub mod options;
pub mod resource;
pub mod auth;
pub mod user;
pub mod child;
pub mod common;
pub mod tool;

use std::sync::{Arc, Mutex};
use auth::KidsnoteAuthSdk;
use resource::KidsnoteResourceSdk;
use child::KidsnoteChildSdk;
use options::KidsnoteOptions;
use user::KidsnoteUserSdk;
use tool::image_tool;

pub struct KidsnoteSdk {
    options: Arc<Mutex<KidsnoteOptions>>,
    auth: KidsnoteAuthSdk,
    resource: KidsnoteResourceSdk,
    user: KidsnoteUserSdk,
    child: KidsnoteChildSdk,
}

impl KidsnoteSdk {
    pub fn new(config:KidsnoteOptions) -> KidsnoteSdk {
        let config_arc = Arc::new(Mutex::new(config));
        let auth = KidsnoteAuthSdk::new(Arc::clone(&config_arc));
        let resource = KidsnoteResourceSdk::new(Arc::clone(&config_arc));
        let user: KidsnoteUserSdk = KidsnoteUserSdk::new(Arc::clone(&config_arc));
        let child = KidsnoteChildSdk::new(Arc::clone(&config_arc));
        KidsnoteSdk { 
            options: config_arc,
            auth,
            resource,
            user,
            child
        }
    }

    pub fn get_options_clone(&self) -> KidsnoteOptions {
        let options = self.options.lock().unwrap();
        (*options).clone()
    }

    pub fn is_refresh_token(&self) -> bool {  
        let options = self.options.lock().unwrap();
        options.is_refresh_token()
    }

    pub fn set_refresh_token(&self, refresh_token:String, user_id:Option<String>) {
        let mut options = self.options.lock().unwrap();
        options.set_refresh_token(refresh_token);
        if let Some(user_id) = user_id { 
            options.set_user_id(user_id);
        }
    }

    /// auth sdk
    pub fn auth(&mut self) -> &mut KidsnoteAuthSdk { &mut self.auth }

    // resource sdk
    pub fn resource(&mut self) -> &mut KidsnoteResourceSdk { &mut self.resource }

    /// child sdk
    pub fn child(&mut self) -> &mut KidsnoteChildSdk { &mut self.child }

    /// user sdk
    pub fn user(&mut self) -> &mut KidsnoteUserSdk { &mut self.user }

    ///
    pub fn get_client_id(&self) -> String { 
        let options = &self.options.lock().unwrap();
        options.get_client_id()
    }
    
    ///
    pub fn get_host(&self) -> String { 
        let options = &self.options.lock().unwrap();
        options.get_host()
    }
    
}


//#[cfg(tests)]
mod tests {

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn text_to_image_test() {
        let text = String::from("안녕하세요 ^^ \n\n어느덧 2023년도 연말이네요~\n연말 마무리는 잘 하고 계신가요?\n\n다름이 아니오라 1월~입학 전까지\n어린이집 주요일정에 대해\n안내드리오니 확인하시고\n미리 일정을 조정하시어\n참석해주시면 감사하겠습니다 ^^\n\n연말 잘 보내시고\n모두 새해 복 많이 받으세요 😊\n\n내년에 만나요 ~ 💖");

        let texts: Vec<&str> = text.split('\n')
            .map(|s| s.trim())
            .collect();
        
        crate::tool::image_tool::ImageTool::text_to_image("[2023-01-01] 브라운스톤어린이집 알림장", &texts, "./test.png");
    }
}