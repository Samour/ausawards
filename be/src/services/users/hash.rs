use crate::errors::ServerError;
use pwhash::bcrypt;
use warp::Rejection;

pub trait HashService {
  fn hash_pw(&self, password: &str) -> Result<String, Rejection>;
}

pub struct HashServiceImpl {}

impl HashServiceImpl {
  pub fn new() -> HashServiceImpl {
    HashServiceImpl {}
  }
}

impl HashService for HashServiceImpl {
  fn hash_pw(&self, password: &str) -> Result<String, Rejection> {
    match bcrypt::hash(password) {
      Ok(s) => Ok(s),
      Err(e) => {
        log::error!("Failure encrypting string: {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}
