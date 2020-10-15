use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
  pub sub: String,
  #[serde(rename = "loginId")]
  pub login_id: String,
  #[serde(rename = "sessionId")]
  pub session_id: String,
  pub permissions: Vec<String>,
  pub iat: i64,
  pub exp: i64,
}
