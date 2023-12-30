use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceImageResponse {
   pub id: u64,
   pub access_key: String,
   pub original_file_name: String,
   pub file_size: i32,
   pub width: i32,
   pub height: i32,
   /// original image url
   pub original: String,
   /// large image url
   pub large: String,
   /// small image url
   pub small: String,
}