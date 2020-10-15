use crate::converters::user::UserConverterImpl;
use crate::converters::UserConverter;
use crate::domain::AppConfig;
use crate::filters::auth::AuthenticationFilterImpl;
use crate::filters::AuthenticationFilter;
use crate::repositories::role::RoleRepositoryImpl;
use crate::repositories::session::UserSessionRepositoryImpl;
use crate::repositories::users::UsersRepositoryImpl;
use crate::repositories::{RoleRepository, UserSessionRepository, UsersRepository};
use crate::routes;
use crate::services::config::FileConfigService;
use crate::services::users::{
  HashServiceImpl, RolesServiceImpl, SessionServiceImpl, TokenServiceImpl, UsersServiceImpl,
};
use crate::services::{
  ConfigService, HashService, RolesService, SessionService, TokenService, UsersService,
};
use mongodb::{Client, Database};
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::Reply;

const COLLECTION_USERS: &str = "Users";
const COLLECTION_SESSIONS: &str = "UserSessions";
const COLLECTION_ROLES: &str = "Roles";

pub struct AppManager {}

impl AppManager {
  pub async fn build() -> BoxedFilter<(impl Reply,)> {
    let config_service = AppManager::config_service();
    let database = AppManager::database(config_service.get_config()).await;
    let hash_service = AppManager::hash_service();
    let users_service =
      AppManager::users_service(Arc::clone(&hash_service), Database::clone(&database));
    let token_service = AppManager::token_service(Arc::clone(&config_service));
    let session_service = AppManager::session_service(
      Arc::clone(&config_service),
      hash_service,
      Arc::clone(&token_service),
      Arc::clone(&users_service),
      database,
    );
    let authentication_filter = AppManager::authentication_filter(token_service);

    AppManager::router(
      &authentication_filter,
      config_service,
      users_service,
      session_service,
    )
  }

  fn config_service() -> Arc<dyn ConfigService + Send + Sync> {
    let mut file_config_service = FileConfigService::new("resources/config.json");
    if let Err(e) = file_config_service.read_config() {
      log::error!("Error attempting to load file: {:?}", e);
      panic!();
    }

    Arc::new(file_config_service)
  }

  async fn database(config: AppConfig) -> Database {
    match Client::with_uri_str(&config.mongo.uri).await {
      Ok(d) => d.database(&config.mongo.database),
      Err(e) => {
        log::error!("Error attempting to connect to database {:?}", e);
        panic!();
      }
    }
  }

  fn user_converter() -> Arc<dyn UserConverter + Send + Sync> {
    Arc::new(UserConverterImpl::new())
  }

  fn users_repository(database: Database) -> Arc<dyn UsersRepository + Send + Sync> {
    Arc::new(UsersRepositoryImpl::new(
      database.collection(COLLECTION_USERS),
    ))
  }

  fn session_repository(database: Database) -> Arc<dyn UserSessionRepository + Send + Sync> {
    Arc::new(UserSessionRepositoryImpl::new(
      database.collection(COLLECTION_SESSIONS),
    ))
  }

  fn role_repository(database: Database) -> Arc<dyn RoleRepository + Send + Sync> {
    Arc::new(RoleRepositoryImpl::new(
      database.collection(COLLECTION_ROLES),
    ))
  }

  fn authentication_filter(
    token_service: Arc<dyn TokenService + Send + Sync>,
  ) -> Box<dyn AuthenticationFilter> {
    Box::new(AuthenticationFilterImpl::new(token_service))
  }

  fn hash_service() -> Arc<dyn HashService + Send + Sync> {
    Arc::new(HashServiceImpl::new())
  }

  fn token_service(
    config_service: Arc<dyn ConfigService + Send + Sync>,
  ) -> Arc<dyn TokenService + Send + Sync> {
    Arc::new(TokenServiceImpl::new(config_service))
  }

  fn users_service(
    hash_service: Arc<dyn HashService + Send + Sync>,
    database: Database,
  ) -> Arc<dyn UsersService + Send + Sync> {
    let user_converter = AppManager::user_converter();
    let users_repository = AppManager::users_repository(database);
    Arc::new(UsersServiceImpl::new(
      user_converter,
      hash_service,
      users_repository,
    ))
  }

  fn roles_service(database: Database) -> Arc<dyn RolesService + Send + Sync> {
    let role_repository = AppManager::role_repository(database);
    Arc::new(RolesServiceImpl::new(role_repository))
  }

  fn session_service(
    config_service: Arc<dyn ConfigService + Send + Sync>,
    hash_service: Arc<dyn HashService + Send + Sync>,
    token_service: Arc<dyn TokenService + Send + Sync>,
    users_service: Arc<dyn UsersService + Send + Sync>,
    database: Database,
  ) -> Arc<dyn SessionService + Send + Sync> {
    let roles_service = AppManager::roles_service(Database::clone(&database));
    let session_repository = AppManager::session_repository(database);

    Arc::new(SessionServiceImpl::new(
      config_service,
      hash_service,
      token_service,
      users_service,
      roles_service,
      session_repository,
    ))
  }

  fn router(
    authentication_filter: &Box<dyn AuthenticationFilter>,
    config_service: Arc<dyn ConfigService + Send + Sync>,
    users_service: Arc<dyn UsersService + Send + Sync>,
    session_service: Arc<dyn SessionService + Send + Sync>,
  ) -> BoxedFilter<(impl Reply,)> {
    routes::build(
      authentication_filter,
      config_service,
      users_service,
      session_service,
    )
  }
}
