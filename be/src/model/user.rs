use serde::{Deserialize, Serialize};

pub const USER_TYPE_ADMIN: &str = "ADMIN";

#[derive(Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "userType")]
  pub user_type: String,
  #[serde(rename = "companyId")]
  pub company_id: Option<String>,
  #[serde(rename = "loginId")]
  pub login_id: String,
  pub password: String,
  #[serde(rename = "roleIds")]
  pub role_ids: Vec<String>,
}
