use crate::converters::UserConverter;
use crate::dto::{CreateSystemAdminUserRequest, UserDto};
use crate::errors::ServerError;
use crate::model::user::USER_TYPE_ADMIN;
use crate::model::User;
use crate::repositories::UsersRepository;
use async_trait::async_trait;
use pwhash::bcrypt;
use std::sync::Arc;
use uuid::Uuid;
use warp::Rejection;

#[async_trait]
pub trait UsersService {
  async fn create_admin_user(
    &self,
    user: &CreateSystemAdminUserRequest,
  ) -> Result<UserDto, Rejection>;
}

pub struct UsersServiceImpl {
  user_converter: Arc<dyn UserConverter + Send + Sync>,
  users_repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl UsersServiceImpl {
  pub fn new(
    user_converter: Arc<dyn UserConverter + Send + Sync>,
    users_repository: Arc<dyn UsersRepository + Send + Sync>,
  ) -> UsersServiceImpl {
    UsersServiceImpl {
      user_converter,
      users_repository,
    }
  }

  fn hash_pw(&self, password: &String) -> Result<String, Rejection> {
    match bcrypt::hash(password) {
      Ok(s) => Ok(s),
      Err(e) => {
        log::error!("Failure encrypting password: {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}

#[async_trait]
impl UsersService for UsersServiceImpl {
  async fn create_admin_user(
    &self,
    user: &CreateSystemAdminUserRequest,
  ) -> Result<UserDto, Rejection> {
    log::info!("Creating user with loginId={}", user.login_id);
    let user = User {
      id: Uuid::new_v4().to_hyphenated().to_string(),
      user_type: String::from(USER_TYPE_ADMIN),
      company_id: None,
      login_id: user.login_id.clone(),
      password: self.hash_pw(&user.password)?,
      role_ids: user.role_ids.clone(),
    };
    self.users_repository.save(&user).await?;

    log::info!(
      "User successfully created with id={}, loginId={}",
      user.id,
      user.login_id
    );
    Ok(self.user_converter.to_dto(&user))
  }
}
