use crate::dto::error;
use crate::dto::ErrorResponse;
use crate::errors::{AuthenticationError, AuthorizationError, ValidationError};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn handler(err: Rejection) -> Result<impl Reply, Infallible> {
  let status;
  let code;

  if err.is_not_found() {
    status = StatusCode::NOT_FOUND;
    code = error::ERR_NOT_FOUND;
  } else if let Some(_) = err.find::<AuthenticationError>() {
    status = StatusCode::UNAUTHORIZED;
    code = error::ERR_UNAUTHENTICATED;
  } else if let Some(_) = err.find::<AuthorizationError>() {
    status = StatusCode::FORBIDDEN;
    code = error::ERR_UNAUTHORIZED;
  } else if let Some(_) = err.find::<ValidationError>() {
    status = StatusCode::BAD_REQUEST;
    code = error::ERR_INVALID_PARAMETERS;
  } else {
    status = StatusCode::INTERNAL_SERVER_ERROR;
    code = error::ERR_UNKNOWN;
  }

  Ok(warp::reply::with_status(
    warp::reply::json(&ErrorResponse { code }),
    status,
  ))
}
