use crate::domain::AppConfig;


pub trait ConfigService {
  fn get_config(&self) -> AppConfig;
}

pub struct FileConfigService {
  fname: &'static str,
  config: Option<AppConfig>,
}

impl FileConfigService {
  pub fn new(fname: &'static str) -> FileConfigService {
    FileConfigService {
      fname,
      config: None,
    }
  }

  pub fn read_config(&mut self) -> Result<(), std::io::Error> {
    log::debug!("Loading config from file '{}'", self.fname);
    let content = std::fs::read_to_string(&self.fname)?;
    let config: AppConfig = serde_json::from_str(&content)?;
    self.config = Some(config);

    Ok(())
  }
}

impl ConfigService for FileConfigService {
  fn get_config(&self) -> AppConfig {
    self.config.as_ref().unwrap().clone()
  }
}
