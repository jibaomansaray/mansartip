use actix_web::{Scope };
use crate::country::country_controllers::{
  get_all_countries,
  get_country_by_id
};


pub fn register_routes(scope: Scope) -> Scope {
  scope.service(get_all_countries)
    .service(get_country_by_id)
}
