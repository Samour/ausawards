use crate::services::{ConfigService, SessionService, UsersService};
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod health;
mod session;
mod users;

pub fn build(
  config_service: Arc<dyn ConfigService + Send + Sync>,
  users_service: Arc<dyn UsersService + Send + Sync>,
  session_service: Arc<dyn SessionService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  health::route(config_service)
    .or(users::route(users_service))
    .or(session::route(session_service))
    .boxed()
}
