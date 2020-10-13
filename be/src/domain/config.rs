use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MongoConfig {
  pub uri: String,
  pub database: String,
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
  pub app_name: String,
  pub mongo: MongoConfig,
}
