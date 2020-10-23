use crate::model::Role;
use crate::repositories::common::RepositoryMethods;
use async_trait::async_trait;
use mongodb::Collection;
use warp::Rejection;

#[async_trait]
pub trait RoleRepository {
  async fn find_by_ids(&self, role_ids: Vec<&str>) -> Result<Vec<Role>, Rejection>;
}

pub struct RoleRepositoryImpl {
  collection: Collection,
}

impl RoleRepositoryImpl {
  pub fn new(collection: Collection) -> RoleRepositoryImpl {
    RoleRepositoryImpl { collection }
  }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
  async fn find_by_ids(&self, role_ids: Vec<&str>) -> Result<Vec<Role>, Rejection> {
    RepositoryMethods::find_by_ids(&self.collection, role_ids).await
  }
}
