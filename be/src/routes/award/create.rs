use crate::dto::CreateAwardRequest;
use crate::filters::AuthenticationFilter;
use crate::services::AwardService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn route(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  award_service: Arc<dyn AwardService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
  warp::path::end()
    .and(warp::post())
    .and(authentication_filter.permissioned("createAward"))
    .and(warp::body::json())
    .and_then(move |award: CreateAwardRequest| {
      let as_local = Arc::clone(&award_service);
      async move { as_local.create_award(award).await.map(|_| warp::reply()) }
    })
    .boxed()
}
