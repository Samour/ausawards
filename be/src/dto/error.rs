use serde::Serialize;

pub const ERR_UNKNOWN: &str = "ERR1000";
pub const ERR_UNAUTHENTICATED: &str = "ERR1001";
pub const ERR_UNAUTHORIZED: &str = "ERR1002";
pub const ERR_NOT_FOUND: &str = "ERR1003";
pub const ERR_INVALID_PARAMETERS: &str = "ERR1004";

#[derive(Serialize)]
pub struct ErrorResponse {
  pub code: &'static str,
}
