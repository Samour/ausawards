use crate::filters::AuthenticationFilter;
use crate::handlers;
use crate::services::{ConfigService, SessionService, UsersService};
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod health;
mod session;
mod users;

pub fn build(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  config_service: Arc<dyn ConfigService + Send + Sync>,
  users_service: Arc<dyn UsersService + Send + Sync>,
  session_service: Arc<dyn SessionService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  health::route(authentication_filter, config_service)
    .or(users::route(authentication_filter, users_service))
    .or(session::route(authentication_filter, session_service))
    .recover(handlers::error::handler)
    .boxed()
}
