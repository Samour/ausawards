use crate::filters::AuthenticationFilter;
use crate::services::UsersService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod create;

pub fn route(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  users_service: Arc<dyn UsersService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  warp::path!("users" / ..)
    .and(create::route(authentication_filter, users_service))
    .boxed()
}
