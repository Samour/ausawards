use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Role {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub permissions: Vec<String>,
}
