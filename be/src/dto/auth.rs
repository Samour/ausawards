use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LogInRequest {
  #[serde(rename = "loginId")]
  pub login_id: String,
  #[serde(rename = "password")]
  pub password: String,
}

#[derive(Serialize)]
pub struct LogInResponse {
  #[serde(rename = "sessionId")]
  pub session_id: String,
  #[serde(rename = "sessionSecret")]
  pub session_secret: String,
  pub token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
  #[serde(rename = "sessionId")]
  pub session_id: String,
  #[serde(rename = "sessionSecret")]
  pub session_secret: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
  pub token: String,
}
