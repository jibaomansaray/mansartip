use actix_web::web;

use crate::app::app_state::AppState;

use super::{country_repo_service::CountryRepoService, country_service::CountryService};

pub fn configure(config: &mut web::ServiceConfig, app_data: web::Data<AppState>) {
    let country_repo = CountryRepoService::new(app_data.db_pool.clone());
    let country_service = web::Data::new(CountryService::new(country_repo));

    config.app_data(country_service);
}
