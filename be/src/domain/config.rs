use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MongoConfig {
  pub uri: String,
  pub database: String,
}

#[derive(Deserialize, Clone)]
pub struct SessionConfig {
  pub secret_length: usize,
  pub lifetime_sec: i64,
}

#[derive(Deserialize, Clone)]
pub struct JwtConfig {
  pub secret: String,
  pub exp_sec: i64,
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
  pub app_name: String,
  pub mongo: MongoConfig,
  pub session: SessionConfig,
  pub jwt: JwtConfig,
}
