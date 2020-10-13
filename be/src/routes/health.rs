use crate::services::ConfigService;
use serde::Serialize;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

#[derive(Serialize)]
struct Response {
  name: String,
}

pub fn route(
  config_service: Arc<dyn ConfigService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  warp::path!("health")
    .and(warp::get())
    .map(move || {
      warp::reply::json(&Response {
        name: config_service.get_config().app_name,
      })
    })
    .boxed()
}
