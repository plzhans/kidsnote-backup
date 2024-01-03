lazy_static::lazy_static! {
    pub static ref STATIC_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub fn get_client() -> &'static STATIC_CLIENT {
    &STATIC_CLIENT
}

// pub fn get_client() -> reqwest::Client {
//     reqwest::Client::new()
// }