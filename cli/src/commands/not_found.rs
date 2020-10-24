use crate::args::ArgsParser;
use crate::commands::Command;
use crate::domain::Config;

pub struct CommandNotFound {
  name: String,
}

impl CommandNotFound {
  pub fn new(name: &str) -> CommandNotFound {
    CommandNotFound {
      name: String::from(name),
    }
  }
}

impl Command for CommandNotFound {
  fn execute(&self, _: ArgsParser, _: Config) -> () {
    println!("Could not recognise command '{}'", self.name);
  }
}
