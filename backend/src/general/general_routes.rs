use actix_web::{Scope };

use super::general_controllers:: {
  index_page
};

pub fn register_api_routes(scope: Scope) -> Scope {
  scope
}

pub fn register_public_routes(scope: Scope) -> Scope {
  scope.service(index_page)
}