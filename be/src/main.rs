use log::LevelFilter;
use simple_logger::SimpleLogger;

mod converters;
mod domain;
mod dto;
mod errors;
mod manager;
mod model;
mod repositories;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .with_module_level("ausawards_be", LevelFilter::Debug)
        .init()
        .unwrap();

    let route = manager::AppManager::build().await;

    log::info!("Starting server on port 3030");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
