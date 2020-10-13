use crate::converters::user::UserConverterImpl;
use crate::converters::UserConverter;
use crate::domain::AppConfig;
use crate::repositories::users::UsersRepositoryImpl;
use crate::repositories::UsersRepository;
use crate::routes;
use crate::services::config::FileConfigService;
use crate::services::users::UsersServiceImpl;
use crate::services::{ConfigService, UsersService};
use mongodb::{Client, Database};
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::Reply;

pub struct AppManager {}

impl AppManager {
  pub async fn build() -> BoxedFilter<(impl Reply,)> {
    let config_service = Arc::new(AppManager::config_service());
    let database = AppManager::database(config_service.get_config()).await;
    let users_service = Arc::new(AppManager::users_service(database));
    AppManager::router(config_service, users_service)
  }

  fn config_service() -> impl ConfigService {
    let mut file_config_service = FileConfigService::new("resources/config.json");
    if let Err(e) = file_config_service.read_config() {
      log::error!("Error attempting to load file: {:?}", e);
      panic!();
    }

    file_config_service
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

  fn user_converter() -> impl UserConverter {
    UserConverterImpl::new()
  }

  fn users_repository(database: Database) -> impl UsersRepository {
    UsersRepositoryImpl::new(database.collection("Users"))
  }

  fn users_service(database: Database) -> impl UsersService {
    let user_converter = Arc::new(AppManager::user_converter());
    let users_repository = Arc::new(AppManager::users_repository(database));
    UsersServiceImpl::new(user_converter, users_repository)
  }

  fn router(
    config_service: Arc<dyn ConfigService + Send + Sync>,
    users_service: Arc<dyn UsersService + Send + Sync>,
  ) -> BoxedFilter<(impl Reply,)> {
    routes::build(config_service, users_service)
  }
}
