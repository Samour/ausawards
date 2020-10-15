use warp::reject::Reject;

#[derive(Debug)]
pub struct ValidationError {}

impl ValidationError {
  pub fn new() -> ValidationError {
    ValidationError {}
  }
}

impl Reject for ValidationError {}
