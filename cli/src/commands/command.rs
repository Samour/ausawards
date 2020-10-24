use super::not_found::CommandNotFound;
use crate::domain::Config;
use crate::ArgsParser;
use std::collections::HashMap;

pub trait Command {
  fn execute(&self, parser: ArgsParser, config: Config) -> ();
}

pub struct SelectCommand {
  commands: HashMap<String, Box<dyn Command>>,
}

impl SelectCommand {
  pub fn new() -> SelectCommand {
    SelectCommand {
      commands: HashMap::new(),
    }
  }

  pub fn command<T>(&mut self, command_str: &str, command: T) -> &mut SelectCommand
  where
    T: Command + 'static,
  {
    self
      .commands
      .insert(String::from(command_str), Box::new(command));

    self
  }

  pub fn select(&mut self, command: &str) -> Box<dyn Command> {
    if let Some(c) = self.commands.remove(command) {
      c
    } else {
      Box::new(CommandNotFound::new(command))
    }
  }
}
