use crate::converters::UserConverter;
use crate::dto::{CreateSystemAdminUserRequest, UserDto};
use crate::model::user::USER_TYPE_ADMIN;
use crate::model::User;
use crate::repositories::UsersRepository;
use crate::services::HashService;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use warp::Rejection;

#[async_trait]
pub trait UsersService {
  async fn create_admin_user(
    &self,
    user: &CreateSystemAdminUserRequest,
  ) -> Result<UserDto, Rejection>;
  async fn get_user(&self, user_id: &str) -> Result<Option<User>, Rejection>;
  async fn get_user_by_login_id(&self, login_id: &str) -> Result<Option<User>, Rejection>;
}

pub struct UsersServiceImpl {
  user_converter: Arc<dyn UserConverter + Send + Sync>,
  hash_service: Arc<dyn HashService + Send + Sync>,
  users_repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl UsersServiceImpl {
  pub fn new(
    user_converter: Arc<dyn UserConverter + Send + Sync>,
    hash_service: Arc<dyn HashService + Send + Sync>,
    users_repository: Arc<dyn UsersRepository + Send + Sync>,
  ) -> UsersServiceImpl {
    UsersServiceImpl {
      user_converter,
      hash_service,
      users_repository,
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
      password: self.hash_service.hash_pw(&user.password)?,
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

  async fn get_user(&self, user_id: &str) -> Result<Option<User>, Rejection> {
    self.users_repository.find_by_id(user_id).await
  }

  async fn get_user_by_login_id(&self, login_id: &str) -> Result<Option<User>, Rejection> {
    self.users_repository.find_by_login_id(login_id).await
  }
}
