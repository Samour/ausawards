use crate::dto::UserDto;
use crate::model::User;

pub trait UserConverter {
  fn to_dto(&self, user: &User) -> UserDto;
}

pub struct UserConverterImpl {}

impl UserConverterImpl {
  pub fn new() -> UserConverterImpl {
    UserConverterImpl {}
  }
}

impl UserConverter for UserConverterImpl {
  fn to_dto(&self, user: &User) -> UserDto {
    UserDto {
      id: user.id.clone(),
      user_type: user.user_type.clone(),
      company_id: user.company_id.clone(),
      login_id: user.login_id.clone(),
      role_ids: user.role_ids.clone(),
    }
  }
}
