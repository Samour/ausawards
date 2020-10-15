use crate::domain::UserAuth;
use crate::errors::{AuthenticationError, AuthorizationError};
use crate::services::TokenService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection};

const AUTH_PREFIX: &str = "Bearer ";

pub trait AuthenticationFilter {
  fn auth_user(&self) -> BoxedFilter<(UserAuth,)>;
  fn authenticated(&self) -> BoxedFilter<()>;
  fn permissioned(&self, permission: &'static str) -> BoxedFilter<()>;
}

pub struct AuthenticationFilterImpl {
  token_service: Arc<dyn TokenService + Send + Sync>,
}

impl AuthenticationFilterImpl {
  pub fn new(token_service: Arc<dyn TokenService + Send + Sync>) -> AuthenticationFilterImpl {
    AuthenticationFilterImpl { token_service }
  }
}

impl AuthenticationFilter for AuthenticationFilterImpl {
  fn auth_user(&self) -> BoxedFilter<(UserAuth,)> {
    let token_service = Arc::clone(&self.token_service);
    warp::header("Authorization")
      .and_then(move |h: String| {
        let ts_local = Arc::clone(&token_service);
        async move {
          if !h.starts_with(AUTH_PREFIX) {
            return Err(warp::reject::custom(AuthenticationError::new()));
          }
          ts_local.parse_token(&h[AUTH_PREFIX.len()..])
        }
      })
      .boxed()
  }

  fn authenticated(&self) -> BoxedFilter<()> {
    self
      .auth_user()
      .and_then(|_| async { Ok(()) as Result<(), Rejection> })
      .untuple_one()
      .boxed()
  }

  fn permissioned(&self, permission: &'static str) -> BoxedFilter<()> {
    self
      .auth_user()
      .and_then(move |u: UserAuth| async move {
        for p in u.permissions {
          if p == permission {
            return Ok(());
          }
        }

        Err(warp::reject::custom(AuthorizationError::new()))
      })
      .untuple_one()
      .boxed()
  }
}
