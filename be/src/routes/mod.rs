use crate::services::{ConfigService, UsersService};
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod health;
mod users;

pub fn build(
  config_service: Arc<dyn ConfigService + Send + Sync>,
  users_service: Arc<dyn UsersService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  health::route(config_service)
    .or(users::route(users_service))
    .boxed()
}
