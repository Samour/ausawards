use warp::reject::Reject;

#[derive(Debug)]
pub struct AuthenticationError {}

impl AuthenticationError {
  pub fn new() -> AuthenticationError {
    AuthenticationError {}
  }
}

impl Reject for AuthenticationError {}
