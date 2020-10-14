use crate::errors::ServerError;
use crate::model::UserSession;
use async_trait::async_trait;
use bson::Document;
use chrono::Utc;
use mongodb::options::UpdateOptions;
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

  fn serialise(&self, session: &UserSession) -> Result<Document, Rejection> {
    match bson::to_document(session) {
      Ok(s) => Ok(s),
      Err(e) => {
        log::error!(
          "Error attempting to serialise UserSession(id={}) {:?}",
          session.id,
          e
        );
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  fn deserialise(&self, doc: Document) -> Result<UserSession, Rejection> {
    match bson::from_document(doc) {
      Ok(u) => Ok(u),
      Err(e) => {
        log::error!("Error attempting to deserialise UserSession {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}

#[async_trait]
impl UserSessionRepository for UserSessionRepositoryImpl {
  async fn save(&self, session: &UserSession) -> Result<(), Rejection> {
    let res = self
      .collection
      .update_one(
        bson::doc! { "_id": &session.id },
        self.serialise(session)?,
        UpdateOptions::builder().upsert(true).build(),
      )
      .await;

    if let Err(e) = res {
      log::error!(
        "Error attempting to save UserSession(id={}) {:?}",
        session.id,
        e
      );
      Err(warp::reject::custom(ServerError::new()))
    } else {
      Ok(())
    }
  }

  async fn find_active_by_id(&self, id: &str) -> Result<Option<UserSession>, Rejection> {
    let res = self
      .collection
      .find_one(
        bson::doc! {
          "_id": id,
          "expireAt": { "$gt": Utc::now() },
          "expired": bson::Bson::Null,
        },
        None,
      )
      .await;
    if let Err(e) = res {
      log::error!(
        "Error while attempting to load UserSession(id={}) {:?}",
        id,
        e
      );
      return Err(warp::reject::custom(ServerError::new()));
    }

    if let Some(s) = res.unwrap() {
      Ok(Some(self.deserialise(s)?))
    } else {
      Ok(None)
    }
  }
}
