use crate::args::ArgsParser;
use crate::commands::Command;
use crate::domain::Config;

pub struct HealthCheckCommand {}

impl HealthCheckCommand {
  pub fn new() -> HealthCheckCommand {
    HealthCheckCommand {}
  }
}

impl Command for HealthCheckCommand {
  fn execute(&self, _: ArgsParser, config: Config) {
    println!("Execute healthcheck against {}", config.remote.url);
  }
}
