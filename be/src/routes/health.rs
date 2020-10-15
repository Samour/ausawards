use crate::filters::AuthenticationFilter;
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
  authentication_filter: &Box<dyn AuthenticationFilter>,
  config_service: Arc<dyn ConfigService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  let cs1 = Arc::clone(&config_service);
  let health = warp::path::end().and(warp::get()).map(move || {
    warp::reply::json(&Response {
      name: cs1.get_config().app_name,
    })
  });

  let health_secure = warp::path!("secure")
    .and(warp::get())
    .and(authentication_filter.authenticated())
    .map(move || {
      warp::reply::json(&Response {
        name: config_service.get_config().app_name,
      })
    });

  warp::path!("health" / ..).and(health.or(health_secure)).boxed()
}
