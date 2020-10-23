use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AwardAlternateId {
  pub id: String,
  #[serde(rename = "type")]
  pub id_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct AwardClassification {
  pub id: String,
  pub title: String,
  pub active: bool,
  pub note: String,
}

#[derive(Deserialize)]
pub struct CreateAwardRequest {
  pub external_id: String,
  pub name: String,
  #[serde(rename = "industryName")]
  pub industry_name: String,
  #[serde(rename = "commonRule")]
  pub common_rule: Option<String>,
  #[serde(rename = "alternateIds")]
  pub alternate_ids: Vec<AwardAlternateId>,
  #[serde(rename = "operativeDate")]
  pub operative_date: DateTime<Utc>,
  #[serde(rename = "expiredDate")]
  pub expired_date: Option<DateTime<Utc>>,
  pub classifications: Vec<AwardClassification>,
}

#[derive(Deserialize)]
pub struct UpdateAwardExpiryDate {
  #[serde(rename = "expiredAt")]
  pub expired_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdateAwardClassificatonStatusRequest {
  pub active: bool,
}

#[derive(Deserialize)]
pub struct UpdateAwardClassificationNoteRequest {
  pub note: String,
}
