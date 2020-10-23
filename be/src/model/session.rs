use crate::repositories::WithId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

pub const SESSION_TYPE_USER: &str = "SESSION_TYPE_USER";

pub const EXPIRY_REASON_LOG_OUT: &str = "LOG_OUT";
pub const EXPIRY_REASON_FORCE_EXPIRE: &str = "FORCE_EXPIRE";

#[derive(Serialize, Deserialize)]
pub struct SessionExpiry {
  #[serde(rename = "expiredBy")]
  pub expired_by: String,
  #[serde(rename = "expiredAt")]
  pub expired_at: DateTime,
  pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSession {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "userId")]
  pub user_id: String,
  #[serde(rename = "sessionType")]
  pub session_type: String,
  #[serde(rename = "sessionSecret")]
  pub session_secret: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime,
  #[serde(rename = "expireAt")]
  pub expire_at: DateTime,
  pub expired: Option<SessionExpiry>,
}

impl WithId for UserSession {
  fn get_id(&self) -> &str {
    &self.id
  }
}
