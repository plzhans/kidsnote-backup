use serde::{Deserialize, Serialize};

use crate::resource::datatypes::ResourceImageResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeInfoResponse {
    pub user: MeInfoUserResponse,
    pub children: Vec<MeInfoChildrenResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeInfoUserResponse {
    pub id: u64,
    pub username: String,
    pub r#type: String,
    pub name: String,
    pub country_code: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub picture: Option<String>,
    pub date_joined: String,
    pub subscription: bool,
    pub subscription_updated_at: String,
    pub third_party_consent: bool,
    pub date_store_allowed: String,
    pub is_staff: bool,
    pub show_change_password: bool,
    pub use_privacy: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeInfoChildrenResponse {
    pub id: u64,
    pub created: String,
    pub name: String,
    pub date_birth: String,
    pub gender: String,
    pub family_type: String,
    pub picture: Option<ResourceImageResponse>, //
    pub parent: MeInfoChildParentResponse,      //
    pub enrollment: Vec<MeInfoChildEnrollment>, //
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeInfoChildParentResponse {
    pub id: u64,
    pub r#type: String,
    pub name: String,
    pub picture: Option<ResourceImageResponse>,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeInfoChildEnrollment {
    pub id: u64,
    pub created: String,
    pub modified: String,
    pub child: u64,
    pub child_id: u64,
    pub child_name: String,
    pub child_birth: String,
    pub child_picture: Option<ResourceImageResponse>,
    pub parent_name: String,
    pub center_id: u64,
    pub center_name: String,
    pub belong_to_class: u64,
    pub class_name: String,
    pub is_approved: bool,
    pub removed_child: bool,
    pub is_extra_parent: bool,
}
