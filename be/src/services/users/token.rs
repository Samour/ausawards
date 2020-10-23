use crate::domain::UserAuth;
use crate::errors::{AuthenticationError, ServerError};
use crate::model::{Role, User, UserSession};
use crate::services::ConfigService;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use warp::Rejection;

pub trait TokenService {
  fn create_token(
    &self,
    user: &User,
    session: &UserSession,
    roles: Vec<&Role>,
  ) -> Result<String, Rejection>;
  fn parse_token(&self, token: &str) -> Result<UserAuth, Rejection>;
}

pub struct TokenServiceImpl {
  enc_key: RwLock<Option<EncodingKey>>,
  dec_key: RwLock<Option<DecodingKey<'static>>>,
  config_service: Arc<dyn ConfigService + Send + Sync>,
}

impl TokenServiceImpl {
  pub fn new(config_service: Arc<dyn ConfigService + Send + Sync>) -> TokenServiceImpl {
    TokenServiceImpl {
      enc_key: RwLock::new(None),
      dec_key: RwLock::new(None),
      config_service,
    }
  }

  fn get_enc_key(&self, key_str: &str) -> Result<EncodingKey, Rejection> {
    {
      let lock = self.enc_key.read();
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

    let lock = self.enc_key.write();
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

  fn get_dec_key(&self) -> Result<DecodingKey, Rejection> {
    {
      let lock = self.dec_key.read();
      if let Err(e) = lock {
        log::error!(
          "Error attempting to obtain read lock on JWT DecodingKey {:?}",
          e
        );
        return Err(warp::reject::custom(ServerError::new()));
      }
      let key = &*lock.unwrap();
      if let Some(k) = key {
        return Ok(k.clone());
      }
    }

    let key = DecodingKey::from_base64_secret(&self.config_service.get_config().jwt.secret);
    if let Err(e) = key {
      log::error!("Error attempting to parse base64 key {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }
    let key = key.unwrap();

    let lock = self.dec_key.write();
    if let Err(e) = lock {
      log::error!(
        "Error attempting to obtain write lock on JWT DecodingKey {:?}",
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
    roles: Vec<&Role>,
  ) -> Result<String, Rejection> {
    let permissions: HashSet<&str> = roles
      .iter()
      .flat_map(|r| r.permissions.iter())
      .map(|p| p as &str)
      .collect();

    let config = self.config_service.get_config().jwt;

    let iat = Utc::now().timestamp();
    let exp = iat + config.exp_sec;

    let spec = UserAuth {
      sub: user.id.clone(),
      login_id: user.login_id.clone(),
      session_id: session.id.clone(),
      permissions: permissions.into_iter().map(String::from).collect(),
      iat,
      exp,
    };

    let tok = jsonwebtoken::encode(
      &Header::new(Algorithm::HS256),
      &spec,
      &self.get_enc_key(&config.secret)?,
    );

    match tok {
      Ok(t) => Ok(t),
      Err(e) => {
        log::error!("Error attempting to construct JWT {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  fn parse_token(&self, token: &str) -> Result<UserAuth, Rejection> {
    match jsonwebtoken::decode(
      token,
      &self.get_dec_key()?,
      &Validation::new(Algorithm::HS256),
    ) {
      Ok(u) => Ok(u.claims),
      Err(_) => Err(warp::reject::custom(AuthenticationError::new())),
    }
  }
}
