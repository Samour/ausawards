use crate::errors::ServerError;
use bson::Document;
use mongodb::options::UpdateOptions;
use mongodb::Collection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::stream::StreamExt;
use warp::Rejection;

pub trait WithId {
  fn get_id(&self) -> &str;
}

pub struct RepositoryMethods {}

impl RepositoryMethods {
  pub fn deserialise<T>(doc: Document) -> Result<T, Rejection>
  where
    T: DeserializeOwned,
  {
    match bson::from_document(doc) {
      Ok(u) => Ok(u),
      Err(e) => {
        log::error!("Error attempting to deserialise Record {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  pub fn serialise<T>(model: &T) -> Result<Document, Rejection>
  where
    T: Serialize,
  {
    match bson::to_document(model) {
      Ok(d) => Ok(d),
      Err(e) => {
        log::error!("Error serialising record {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  pub async fn save<T>(collection: &Collection, model: &T) -> Result<(), Rejection>
  where
    T: Serialize + WithId,
  {
    let res = collection
      .update_one(
        bson::doc! { "_id": model.get_id() },
        RepositoryMethods::serialise(model)?,
        UpdateOptions::builder().upsert(true).build(),
      )
      .await;

    if let Err(e) = res {
      log::error!(
        "Error occurred while attempting to save document with id={} {:?}",
        model.get_id(),
        e
      );
      Err(warp::reject::custom(ServerError::new()))
    } else {
      Ok(())
    }
  }

  pub async fn find_one<T>(collection: &Collection, query: Document) -> Result<Option<T>, Rejection>
  where
    T: DeserializeOwned,
  {
    let res = collection.find_one(query, None).await;
    if let Err(e) = res {
      log::error!("Error executing DB query {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    if let Some(u) = res.unwrap() {
      Ok(Some(RepositoryMethods::deserialise(u)?))
    } else {
      Ok(None)
    }
  }

  pub async fn find_by_id<T>(collection: &Collection, id: &str) -> Result<Option<T>, Rejection>
  where
    T: DeserializeOwned,
  {
    RepositoryMethods::find_one(collection, bson::doc! { "_id": id }).await
  }

  pub async fn find<T>(collection: &Collection, query: Document) -> Result<Vec<T>, Rejection>
  where
    T: DeserializeOwned,
  {
    let res = collection.find(query, None).await;

    if let Err(e) = res {
      log::error!("Error while attempting to execute DB query {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    let mut res = res.unwrap();
    let mut roles = Vec::new();
    while let Some(doc) = res.next().await {
      match doc {
        Ok(d) => roles.push(RepositoryMethods::deserialise(d)?),
        Err(e) => {
          log::error!("Error while attempting to load records from DB {:?}", e);
          return Err(warp::reject::custom(ServerError::new()));
        }
      }
    }
    Ok(roles)
  }

  pub async fn find_by_ids<T>(collection: &Collection, ids: Vec<&str>) -> Result<Vec<T>, Rejection>
  where
    T: DeserializeOwned,
  {
    RepositoryMethods::find(collection, bson::doc! { "_id": { "$in": ids } }).await
  }
}
