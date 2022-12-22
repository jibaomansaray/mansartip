use crate::country::country_entities::Country;

use super::country_repo_service::CountryRepoService;

pub struct CountryService {
    repo: CountryRepoService,
}

impl CountryService {
    pub fn new(repo: CountryRepoService) -> Self {
        Self { repo }
    }

    pub async fn get_all_countries(&self) -> Vec<Country> {
        // @todo check permission
        self.repo.all().await
    }

    pub async fn get_country_by_id(&self, id: i32) -> Option<Country> {
        self.repo.find_one_by_id(id).await
    }
}
