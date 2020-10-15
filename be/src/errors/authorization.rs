use warp::reject::Reject;

#[derive(Debug)]
pub struct AuthorizationError {}

impl AuthorizationError {
  pub fn new() -> AuthorizationError {
    AuthorizationError {}
  }
}

impl Reject for AuthorizationError {}
