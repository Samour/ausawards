use crate::repositories::WithId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

pub const ID_TYPE_PRINT_ID: &str = "ORIGINAL_PRINT_ID";
pub const ID_TYPE_ORIG_MATTER: &str = "ORIGINATING_MATTER";

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

#[derive(Serialize, Deserialize)]
pub struct Award {
  #[serde(rename = "_id")]
  pub id: String,
  pub external_id: String,
  pub name: String,
  #[serde(rename = "industryName")]
  pub industry_name: String,
  #[serde(rename = "commonRule")]
  pub common_rule: Option<String>,
  #[serde(rename = "alternateIds")]
  pub alternate_ids: Vec<AwardAlternateId>,
  #[serde(rename = "operativeDate")]
  pub operative_date: DateTime,
  #[serde(rename = "expiredDate")]
  pub expired_date: Option<DateTime>,
  pub classifications: Vec<AwardClassification>,
}

impl WithId for Award {
  fn get_id(&self) -> &str {
    &self.id
  }
}
