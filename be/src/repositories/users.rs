use crate::model::User;
use crate::repositories::common::RepositoryMethods;
use async_trait::async_trait;
use mongodb::Collection;
use warp::Rejection;

#[async_trait]
pub trait UsersRepository {
  async fn save(&self, user: &User) -> Result<(), Rejection>;
  async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, Rejection>;
  async fn find_by_login_id(&self, login_id: &str) -> Result<Option<User>, Rejection>;
}

pub struct UsersRepositoryImpl {
  collection: Collection,
}

impl UsersRepositoryImpl {
  pub fn new(collection: Collection) -> UsersRepositoryImpl {
    UsersRepositoryImpl { collection }
  }
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
  async fn save(&self, user: &User) -> Result<(), Rejection> {
    RepositoryMethods::save(&self.collection, user).await
  }

  async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, Rejection> {
    RepositoryMethods::find_by_id(&self.collection, user_id).await
  }

  async fn find_by_login_id(&self, login_id: &str) -> Result<Option<User>, Rejection> {
    RepositoryMethods::find_one(&self.collection, bson::doc! { "loginId": login_id }).await
  }
}
