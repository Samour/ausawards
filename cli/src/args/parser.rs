use std::collections::HashMap;

pub enum FlagType {
  Boolean,
  Value(Option<String>),
}

pub struct ArgsParser {
  pub commands: Vec<String>,
  flags: HashMap<String, FlagType>,
}

impl ArgsParser {
  pub fn new() -> ArgsParser {
    ArgsParser {
      commands: Vec::new(),
      flags: HashMap::new(),
    }
  }

  pub fn arg_str(&mut self, name: &str) -> () {
    self.flags.insert(String::from(name), FlagType::Value(None));
  }

  pub fn arg_str_default(&mut self, name: &str, value: &str) -> () {
    self.flags.insert(
      String::from(name),
      FlagType::Value(Some(String::from(value))),
    );
  }

  pub fn parse(&mut self) -> () {
    self.commands = Vec::new();

    let mut store_arg_value: Option<String> = None;
    for arg in std::env::args() {
      if let Some(key) = store_arg_value {
        self.flags.insert(key, FlagType::Value(Some(arg)));
        store_arg_value = None;
      } else if arg.starts_with("--") {
        let arg_name = String::from(&arg[2..]);
        if let Some(FlagType::Value(_)) = self.flags.get(&arg_name) {
          store_arg_value = Some(arg_name);
        } else {
          self.flags.insert(arg_name, FlagType::Boolean);
        }
      } else {
        self.commands.push(arg);
      }
    }
  }

  pub fn is_flag(&self, key: &str) -> bool {
    match self.flags.get(key) {
      Some(FlagType::Value(Some(_))) => true,
      Some(FlagType::Boolean) => true,
      _ => false,
    }
  }

  pub fn get_flag(&self, key: &str) -> Option<String> {
    if let Some(FlagType::Value(Some(v))) = self.flags.get(key) {
      Some(v.clone())
    } else {
      None
    }
  }

  pub fn serve_command(&mut self) -> Option<String> {
    if self.commands.len() > 0 {
      Some(self.commands.remove(0))
    } else {
      None
    }
  }
}
