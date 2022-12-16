use actix_web::{Scope };

use super::game_controllers::get_today_matches;

pub fn register_routes(scope: Scope) -> Scope {
  scope.service(get_today_matches)
}