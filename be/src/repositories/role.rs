use crate::errors::ServerError;
use crate::model::Role;
use async_trait::async_trait;
use bson::Document;
use mongodb::Collection;
use tokio::stream::StreamExt;
use warp::Rejection;

#[async_trait]
pub trait RoleRepository {
  async fn find_by_ids(&self, role_ids: &Vec<&str>) -> Result<Vec<Role>, Rejection>;
}

pub struct RoleRepositoryImpl {
  collection: Collection,
}

impl RoleRepositoryImpl {
  pub fn new(collection: Collection) -> RoleRepositoryImpl {
    RoleRepositoryImpl { collection }
  }

  fn deserialise(&self, doc: Document) -> Result<Role, Rejection> {
    match bson::from_document(doc) {
      Ok(u) => Ok(u),
      Err(e) => {
        log::error!("Error attempting to deserialise Role {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
  async fn find_by_ids(&self, role_ids: &Vec<&str>) -> Result<Vec<Role>, Rejection> {
    let res = self
      .collection
      .find(bson::doc! { "_id": { "$in": role_ids } }, None)
      .await;

    if let Err(e) = res {
      log::error!("Error while attempting to load roles from DB {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    let mut res = res.unwrap();
    let mut roles = Vec::new();
    while let Some(doc) = res.next().await {
      match doc {
        Ok(d) => roles.push(self.deserialise(d)?),
        Err(e) => {
          log::error!("Error while attempting to load roles from DB {:?}", e);
          return Err(warp::reject::custom(ServerError::new()));
        }
      }
    }
    Ok(roles)
  }
}
