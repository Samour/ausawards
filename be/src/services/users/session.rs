use crate::dto::{LogInRequest, LogInResponse, RefreshRequest, RefreshResponse};
use crate::errors::{AuthenticationError, ServerError, ValidationError};
use crate::model::session::{EXPIRY_REASON_LOG_OUT, SESSION_TYPE_USER};
use crate::model::{SessionExpiry, User, UserSession};
use crate::repositories::UserSessionRepository;
use crate::services::{ConfigService, HashService, RolesService, TokenService, UsersService};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use pwhash::bcrypt;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use uuid::Uuid;
use warp::Rejection;

#[async_trait]
pub trait SessionService {
  async fn create_session(&self, request: &LogInRequest) -> Result<LogInResponse, Rejection>;
  async fn refresh_token(&self, request: &RefreshRequest) -> Result<RefreshResponse, Rejection>;
  async fn sign_out(&self, session_id: &str) -> Result<(), Rejection>;
}

pub struct SessionServiceImpl {
  config_service: Arc<dyn ConfigService + Send + Sync>,
  hash_service: Arc<dyn HashService + Send + Sync>,
  token_service: Arc<dyn TokenService + Send + Sync>,
  users_service: Arc<dyn UsersService + Send + Sync>,
  roles_service: Arc<dyn RolesService + Send + Sync>,
  session_repository: Arc<dyn UserSessionRepository + Send + Sync>,
}

impl SessionServiceImpl {
  pub fn new(
    config_service: Arc<dyn ConfigService + Send + Sync>,
    hash_service: Arc<dyn HashService + Send + Sync>,
    token_service: Arc<dyn TokenService + Send + Sync>,
    users_service: Arc<dyn UsersService + Send + Sync>,
    roles_service: Arc<dyn RolesService + Send + Sync>,
    session_repository: Arc<dyn UserSessionRepository + Send + Sync>,
  ) -> SessionServiceImpl {
    SessionServiceImpl {
      config_service,
      hash_service,
      token_service,
      users_service,
      roles_service,
      session_repository,
    }
  }

  fn create_secret(&self) -> String {
    rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(self.config_service.get_config().session.secret_length)
      .collect()
  }

  async fn create_token(&self, user: &User, session: &UserSession) -> Result<String, Rejection> {
    let roles = self
      .roles_service
      .get_roles_by_id(&user.role_ids.iter().map(|i| i as &str).collect())
      .await?;

    self
      .token_service
      .create_token(&user, &session, &roles.iter().collect())
  }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
  async fn create_session(&self, request: &LogInRequest) -> Result<LogInResponse, Rejection> {
    let user = self
      .users_service
      .get_user_by_login_id(&request.login_id)
      .await?;
    if let None = user {
      return Err(warp::reject::custom(AuthenticationError::new()));
    }

    let user = user.unwrap();
    if !bcrypt::verify(&request.password, &user.password) {
      return Err(warp::reject::custom(AuthenticationError::new()));
    }

    let session_secret = self.create_secret();
    let created_at = Utc::now();
    let expire_at =
      created_at + Duration::seconds(self.config_service.get_config().session.lifetime_sec);
    let session = UserSession {
      id: Uuid::new_v4().to_hyphenated().to_string(),
      user_id: user.id.clone(),
      session_type: String::from(SESSION_TYPE_USER),
      session_secret: self.hash_service.hash_pw(&session_secret)?,
      created_at: bson::DateTime::from(created_at),
      expire_at: bson::DateTime::from(expire_at),
      expired: None,
    };

    self.session_repository.save(&session).await?;

    Ok(LogInResponse {
      session_id: session.id.clone(),
      session_secret,
      token: self.create_token(&user, &session).await?,
    })
  }

  async fn refresh_token(&self, request: &RefreshRequest) -> Result<RefreshResponse, Rejection> {
    let session = self
      .session_repository
      .find_active_by_id(&request.session_id)
      .await?;
    if let None = session {
      log::debug!("Could not locate session {}", request.session_id);
      return Err(warp::reject::custom(AuthenticationError::new()));
    }
    let session = session.unwrap();

    if !bcrypt::verify(&request.session_secret, &session.session_secret) {
      log::debug!("Invalid key for session {}", request.session_id);
      return Err(warp::reject::custom(AuthenticationError::new()));
    }

    let user = self.users_service.get_user(&session.user_id).await?;
    if let None = user {
      log::error!(
        "Could not find User(id={}), but Session(id={}) is valid",
        &session.user_id,
        &session.id
      );
      return Err(warp::reject::custom(ServerError::new()));
    }

    Ok(RefreshResponse {
      token: self.create_token(&user.unwrap(), &session).await?,
    })
  }

  async fn sign_out(&self, session_id: &str) -> Result<(), Rejection> {
    let session = self
      .session_repository
      .find_active_by_id(session_id)
      .await?;

    if let None = session {
      log::debug!(
        "Attempted to sign out session {}, but no active session could be found",
        session_id
      );
      return Err(warp::reject::custom(ValidationError::new()));
    }

    let mut session = session.unwrap();
    session.expired = Some(SessionExpiry {
      expired_by: session.user_id.clone(),
      expired_at: bson::DateTime::from(Utc::now()),
      reason: String::from(EXPIRY_REASON_LOG_OUT),
    });
    self.session_repository.save(&session).await?;

    Ok(())
  }
}
