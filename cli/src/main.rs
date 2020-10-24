mod args;
mod commands;
mod config;
mod domain;

use args::ArgsParser;

const VERSION: &str = "0.0.1-dev";

fn main() {
    let mut parser = ArgsParser::new();
    if let Ok(home) = std::env::var("HOME") {
        parser.arg_str_default("config", &format!("{}/.ausawards/config.yaml", home));
    }

    parser.parse();
    parser.serve_command();

    if parser.is_flag("version") {
        println!("{}", VERSION);
        return;
    }
    let config = config::load_config(parser.get_flag("config"));

    commands::SelectCommand::new()
        .command("healthcheck", commands::HealthCheckCommand::new())
        .select(&parser.serve_command().unwrap_or(String::from("")))
        .execute(parser, config);
}
