use crate::domain::Config;
use serde_yaml;

pub fn load_config(fname: Option<String>) -> Config {
  if let None = fname {
    panic!("Could not find config file");
  }
  let fname = fname.unwrap();

  let content = std::fs::read_to_string(&fname);
  if content.is_err() {
    panic!("Could not load config file at {}", fname);
  }

  let config: Result<Config, _> = serde_yaml::from_str(&content.unwrap());
  if config.is_err() {
    panic!("Error reading config file {}", fname);
  }

  config.unwrap()
}
