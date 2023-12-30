
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::resource::datatypes::ResourceImageResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildReportResponse {
   pub count: i32,
   pub next: Option<String>,
   pub previous: Option<String>,
   pub results: Vec<ChildReportDataResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildReportDataResponse {
    pub id: u64,
    pub created: DateTime<Utc>,
    pub modified: String,
    pub date_written: String,
    pub author: ChildReportAuthorResponse,
    pub author_name: String,
    pub center: u64,
    pub cls: u64,
    pub class_name: String,
    pub child: u64,
    pub child_name: String,
    pub child_picture: Option<ResourceImageResponse>,
    pub is_sent_from_center: bool,
    pub content: String,
    pub weather: String,
    //pub attached_video: Vec<String>,
    pub num_comments: i32,
    pub read_by_me: bool,
    pub read_by_parent: ChildReportReadByParentResponse,
    pub attached_images: Vec<ResourceImageResponse>,
    //pub attached_files: Vec<String>,
    pub thumbnail: Option<String>,
 }

 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct ChildReportAuthorResponse {
    pub id: u64,
    pub r#type: String,
    pub name: String,
    pub picture: Option<ResourceImageResponse>,
    pub username: String,
 }

 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct ChildReportReadByParentResponse {
    pub date_read: String,
 }