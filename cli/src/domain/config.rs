use serde::Deserialize;

#[derive(Deserialize)]
pub struct RemoteConfig {
  pub url: String,
}

#[derive(Deserialize)]
pub struct Config {
  pub remote: RemoteConfig,
}
