use crate::errors::ServerError;
use crate::model::{Role, User, UserSession};
use crate::services::ConfigService;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use warp::Rejection;

#[derive(Serialize)]
struct TokenSpec<'a> {
  sub: &'a str,
  #[serde(rename = "loginId")]
  login_id: &'a str,
  #[serde(rename = "sessionId")]
  session_id: &'a str,
  permissions: Vec<&'a str>,
  iat: i64,
  exp: i64,
}

pub trait TokenService {
  fn create_token(
    &self,
    user: &User,
    session: &UserSession,
    roles: &Vec<&Role>,
  ) -> Result<String, Rejection>;
}

pub struct TokenServiceImpl {
  key: RwLock<Option<EncodingKey>>,
  config_service: Arc<dyn ConfigService + Send + Sync>,
}

impl TokenServiceImpl {
  pub fn new(config_service: Arc<dyn ConfigService + Send + Sync>) -> TokenServiceImpl {
    TokenServiceImpl {
      key: RwLock::new(None),
      config_service,
    }
  }

  fn get_key(&self, key_str: &str) -> Result<EncodingKey, Rejection> {
    {
      let lock = self.key.read();
      if let Err(e) = lock {
        log::error!(
          "Error attempting to obtain read lock on JWT EncodingKey {:?}",
          e
        );
        return Err(warp::reject::custom(ServerError::new()));
      }
      let key = &*lock.unwrap();
      if let Some(k) = key {
        return Ok(k.clone());
      }
    }

    let key = EncodingKey::from_base64_secret(key_str);
    if let Err(e) = key {
      log::error!("Error attempting to parse base64 key {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }
    let key = key.unwrap();

    let lock = self.key.write();
    if let Err(e) = lock {
      log::error!(
        "Error attempting to obtain write lock on JWT EncodingKey {:?}",
        e
      );
      return Err(warp::reject::custom(ServerError::new()));
    }
    *lock.unwrap() = Some(key.clone());

    Ok(key)
  }
}

impl TokenService for TokenServiceImpl {
  fn create_token(
    &self,
    user: &User,
    session: &UserSession,
    roles: &Vec<&Role>,
  ) -> Result<String, Rejection> {
    let permissions: HashSet<&str> = roles
      .iter()
      .flat_map(|r| r.permissions.iter())
      .map(|p| p as &str)
      .collect();

    let config = self.config_service.get_config().jwt;

    let iat = Utc::now().timestamp();
    let exp = iat + config.exp_sec;

    let spec = TokenSpec {
      sub: &user.id,
      login_id: &user.login_id,
      session_id: &session.id,
      permissions: permissions.iter().map(|s| *s).collect(),
      iat,
      exp,
    };

    let tok = jsonwebtoken::encode(
      &Header::new(Algorithm::HS256),
      &spec,
      &self.get_key(&config.secret)?,
    );

    match tok {
      Ok(t) => Ok(t),
      Err(e) => {
        log::error!("Error attempting to construct JWT {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}
