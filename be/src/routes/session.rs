use crate::domain::UserAuth;
use crate::dto::{LogInRequest, RefreshRequest};
use crate::filters::AuthenticationFilter;
use crate::services::SessionService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

pub fn route(
  authentication_filter: &Box<dyn AuthenticationFilter>,
  session_service: Arc<dyn SessionService + Send + Sync>,
) -> BoxedFilter<(impl Reply,)> {
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

  let ss2 = Arc::clone(&session_service);
  let refresh_token = warp::path!("refresh")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |r: RefreshRequest| {
      let ss_local = Arc::clone(&ss2);
      async move {
        ss_local
          .refresh_token(&r)
          .await
          .map(|t| warp::reply::json(&t))
      }
    });

  let sign_out = warp::path::end()
    .and(warp::delete())
    .and(authentication_filter.auth_user())
    .and_then(move |u: UserAuth| {
      let ss_local = Arc::clone(&session_service);
      async move {
        ss_local.sign_out(&u.session_id).await?;
        Ok(warp::reply()) as Result<_, Rejection>
      }
    });

  warp::path!("session" / ..)
    .and(create_session.or(refresh_token).or(sign_out))
    .boxed()
}
