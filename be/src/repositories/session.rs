use crate::model::UserSession;
use crate::repositories::common::RepositoryMethods;
use async_trait::async_trait;
use chrono::Utc;
use mongodb::Collection;
use warp::Rejection;

#[async_trait]
pub trait UserSessionRepository {
  async fn save(&self, session: &UserSession) -> Result<(), Rejection>;
  async fn find_active_by_id(&self, id: &str) -> Result<Option<UserSession>, Rejection>;
}

pub struct UserSessionRepositoryImpl {
  collection: Collection,
}

impl UserSessionRepositoryImpl {
  pub fn new(collection: Collection) -> UserSessionRepositoryImpl {
    UserSessionRepositoryImpl { collection }
  }
}

#[async_trait]
impl UserSessionRepository for UserSessionRepositoryImpl {
  async fn save(&self, session: &UserSession) -> Result<(), Rejection> {
    RepositoryMethods::save(&self.collection, session).await
  }

  async fn find_active_by_id(&self, id: &str) -> Result<Option<UserSession>, Rejection> {
    RepositoryMethods::find_one(
      &self.collection,
      bson::doc! {
        "_id": id,
        "expireAt": { "$gt": Utc::now() },
        "expired": bson::Bson::Null,
      },
    )
    .await
  }
}
