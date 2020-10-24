use crate::args::ArgsParser;
use crate::commands::Command;
use crate::domain::Config;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
  name: String,
}

pub struct HealthCheckCommand {}

impl HealthCheckCommand {
  pub fn new() -> HealthCheckCommand {
    HealthCheckCommand {}
  }
}

impl Command for HealthCheckCommand {
  fn execute(&self, _: ArgsParser, config: Config) -> () {
    let base_url = reqwest::Url::parse(&config.remote.url);
    if base_url.is_err() {
      println!("Malformed URL: {}", config.remote.url);
      return;
    }
    let base_url = base_url.unwrap();

    let response = reqwest::blocking::get(base_url.join("/health").unwrap())
      .ok()
      .map(|r| r.json::<Response>().ok())
      .flatten();
    if let Some(r) = response {
      println!("Successfully connected to {}", r.name);
    } else {
      println!("Could not connect to server");
    }
  }
}
