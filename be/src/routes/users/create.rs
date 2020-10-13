use crate::dto::CreateSystemAdminUserRequest;
use crate::services::UsersService;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn route(users_service: Arc<dyn UsersService + Send + Sync>) -> BoxedFilter<(impl Reply,)> {
  let create_admin = warp::path!("create" / "admin")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |request: CreateSystemAdminUserRequest| {
      let us_local = Arc::clone(&users_service);
      async move {
        us_local
          .create_admin_user(&request)
          .await
          .map(|r| warp::reply::json(&r))
      }
    });

  create_admin.boxed()
}
