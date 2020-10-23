use crate::filters::AuthenticationFilter;
use crate::services::AwardService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

mod create;
mod update;

pub fn route(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  award_service: Arc<dyn AwardService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  warp::path!("awards" / ..)
    .and(
      create::route(authentication_filter, Arc::clone(&award_service))
        .or(update::route(authentication_filter, award_service)),
    )
    .boxed()
}
