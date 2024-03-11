pub mod auth;
pub mod child;
pub mod common;
pub mod options;
pub mod resource;
pub mod tool;
pub mod user;

use auth::KidsnoteAuthSdk;
use child::KidsnoteChildSdk;
use options::KidsnoteOptions;
use resource::KidsnoteResourceSdk;
use std::sync::{Arc, Mutex};
use user::KidsnoteUserSdk;

pub struct KidsnoteSdk {
    options: Arc<Mutex<KidsnoteOptions>>,
    auth: KidsnoteAuthSdk,
    resource: KidsnoteResourceSdk,
    user: KidsnoteUserSdk,
    child: KidsnoteChildSdk,
}

impl KidsnoteSdk {
    pub fn new(config: KidsnoteOptions) -> KidsnoteSdk {
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
            child,
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

    pub fn set_refresh_token(&self, refresh_token: String, user_id: Option<String>) {
        let mut options = self.options.lock().unwrap();
        options.set_refresh_token(refresh_token);
        if let Some(user_id) = user_id {
            options.set_user_id(user_id);
        }
    }

    /// auth sdk
    pub fn auth(&mut self) -> &mut KidsnoteAuthSdk {
        &mut self.auth
    }

    // resource sdk
    pub fn resource(&mut self) -> &mut KidsnoteResourceSdk {
        &mut self.resource
    }

    /// child sdk
    pub fn child(&mut self) -> &mut KidsnoteChildSdk {
        &mut self.child
    }

    /// user sdk
    pub fn user(&mut self) -> &mut KidsnoteUserSdk {
        &mut self.user
    }

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
    mod tool;

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn text_to_image_test() {
        let text = String::from("ㅠㅠ 오늘은 꼭 등원하려하는데.. 이제가 어제 못자서 그런지 안일어나네요.. 오후에 가도 괜찮은걸까요..?");

        let texts: Vec<&str> = text.split('\n').map(|s| s.trim()).collect();

        let file_time = chrono::Utc::now();
        let _result = crate::tool::image_tool::ImageTool::text_to_image_file(
            "[2023-01-01] 브라운스톤어린이집 알림장",
            &Some("활빈당".to_string()),
            "홍길동 엄마",
            &texts,
            "./test.png",
            file_time,
        );
    }
}
