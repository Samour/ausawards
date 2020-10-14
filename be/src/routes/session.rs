use crate::dto::{LogInRequest, RefreshRequest};
use crate::services::SessionService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn route(session_service: Arc<dyn SessionService + Send + Sync>) -> BoxedFilter<(impl Reply,)> {
  let ss1 = Arc::clone(&session_service);
  let create_session = warp::path::end()
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |r: LogInRequest| {
      let ss_local = Arc::clone(&ss1);
      async move {
        ss_local
          .create_session(&r)
          .await
          .map(|s| warp::reply::json(&s))
      }
    });

  let refresh_token = warp::path!("refresh")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |r: RefreshRequest| {
      let ss_local = Arc::clone(&session_service);
      async move {
        ss_local
          .refresh_token(&r)
          .await
          .map(|t| warp::reply::json(&t))
      }
    });

  warp::path!("session" / ..)
    .and(create_session.or(refresh_token))
    .boxed()
}
