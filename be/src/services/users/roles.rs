use crate::model::Role;
use crate::repositories::RoleRepository;
use async_trait::async_trait;
use std::sync::Arc;
use warp::Rejection;

#[async_trait]
pub trait RolesService {
  async fn get_roles_by_id(&self, role_ids: &Vec<&str>) -> Result<Vec<Role>, Rejection>;
}

pub struct RolesServiceImpl {
  role_repository: Arc<dyn RoleRepository + Send + Sync>,
}

impl RolesServiceImpl {
  pub fn new(role_repository: Arc<dyn RoleRepository + Send + Sync>) -> RolesServiceImpl {
    RolesServiceImpl { role_repository }
  }
}

#[async_trait]
impl RolesService for RolesServiceImpl {
  async fn get_roles_by_id(&self, role_ids: &Vec<&str>) -> Result<Vec<Role>, Rejection> {
    self.role_repository.find_by_ids(role_ids).await
  }
}
