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

  fn serialise(&self, user: &User) -> Result<Document, Rejection> {
    match bson::to_document(user) {
      Ok(d) => Ok(d),
      Err(e) => {
        log::error!("Error serialising user record userId={} {:?}", user.id, e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  fn deserialise(&self, doc: Document) -> Result<User, Rejection> {
    match bson::from_document(doc) {
      Ok(u) => Ok(u),
      Err(e) => {
        log::error!("Error attempting to deserialise User {:?}", e);
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
        bson::doc! { "_id": &user.id },
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

  async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, Rejection> {
    let res = self
      .collection
      .find_one(bson::doc! { "_id": user_id }, None)
      .await;
    if let Err(e) = res {
      log::error!("Error reading User(id={}) from DB {:?}", user_id, e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    if let Some(u) = res.unwrap() {
      Ok(Some(self.deserialise(u)?))
    } else {
      Ok(None)
    }
  }

  async fn find_by_login_id(&self, login_id: &str) -> Result<Option<User>, Rejection> {
    let res = self
      .collection
      .find_one(bson::doc! { "loginId": login_id }, None)
      .await;
    if let Err(e) = res {
      log::error!("Error loading User(loginId={}) from DB {:?}", login_id, e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    if let Some(u) = res.unwrap() {
      Ok(Some(self.deserialise(u)?))
    } else {
      Ok(None)
    }
  }
}
