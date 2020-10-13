use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateSystemAdminUserRequest {
  #[serde(rename = "loginId")]
  pub login_id: String,
  pub password: String,
  #[serde(rename = "roleIds")]
  pub role_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct UserDto {
  pub id: String,
  #[serde(rename = "userType")]
  pub user_type: String,
  #[serde(rename = "companyId")]
  pub company_id: Option<String>,
  #[serde(rename = "loginId")]
  pub login_id: String,
  #[serde(rename = "roleIds")]
  pub role_ids: Vec<String>,
}
