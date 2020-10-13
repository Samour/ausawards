use crate::errors::ServerError;
use crate::model::User;
use async_trait::async_trait;
use bson::Document;
use mongodb::options::UpdateOptions;
use mongodb::Collection;
use warp::Rejection;

#[async_trait]
pub trait UsersRepository {
  async fn save(&self, user: &User) -> Result<(), Rejection>;
}

pub struct UsersRepositoryImpl {
  collection: Collection,
}

impl UsersRepositoryImpl {
  pub fn new(collection: Collection) -> UsersRepositoryImpl {
    UsersRepositoryImpl { collection }
  }

  fn serialise(&self, user: &User) -> Result<Document, Rejection> {
    match bson::to_document(user) {
      Ok(d) => Ok(d),
      Err(e) => {
        log::error!("Error serialising user record: {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
  async fn save(&self, user: &User) -> Result<(), Rejection> {
    let res = self
      .collection
      .update_one(
        bson::doc! { "_id": user.id.clone() },
        self.serialise(user)?,
        UpdateOptions::builder().upsert(true).build(),
      )
      .await;

    if let Err(e) = res {
      log::error!(
        "Error occurred while attempting to save User(id={}) {:?}",
        user.id,
        e
      );
      Err(warp::reject::custom(ServerError::new()))
    } else {
      Ok(())
    }
  }
}
