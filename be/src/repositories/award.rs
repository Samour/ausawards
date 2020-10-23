use crate::model::Award;
use crate::repositories::common::RepositoryMethods;
use async_trait::async_trait;
use mongodb::Collection;
use warp::Rejection;

#[async_trait]
pub trait AwardRepository {
  async fn save(&self, award: &Award) -> Result<(), Rejection>;
  async fn find_by_id(&self, id: &str) -> Result<Option<Award>, Rejection>;
}

pub struct AwardRespositoryImpl {
  collection: Collection,
}

impl AwardRespositoryImpl {
  pub fn new(collection: Collection) -> AwardRespositoryImpl {
    AwardRespositoryImpl { collection }
  }
}

#[async_trait]
impl AwardRepository for AwardRespositoryImpl {
  async fn save(&self, award: &Award) -> Result<(), Rejection> {
    RepositoryMethods::save(&self.collection, award).await
  }

  async fn find_by_id(&self, id: &str) -> Result<Option<Award>, Rejection> {
    RepositoryMethods::find_by_id(&self.collection, id).await
  }
}
