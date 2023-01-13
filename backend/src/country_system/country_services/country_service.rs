use super::country_repo_service::{CountryRepoService, CountryRepoServiceTrait};
use crate::country_system::country_entities::CountryEntity;

pub struct CountryService<T = CountryRepoService> {
    repo: T,
}

impl<T> CountryService<T>
where
    T: CountryRepoServiceTrait,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_all_countries(&self) -> Vec<CountryEntity> {
        // @todo check permission
        self.repo.all().await
    }

    pub async fn get_country_by_id(&self, id: i32) -> Option<CountryEntity> {
        // @todo check permission
        self.repo.find_one_by_id(id).await
    }

    pub async fn get_country_by_internal_id(&self, id: &str) -> Option<CountryEntity> {
        // @todo check permission
        self.repo.find_one_by_internal_id(id).await
    }
}
