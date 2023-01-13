use crate::tipping_system::tipping_entities::scoreboard_entity::ScoreboardEntity;

use super::tipping_scoreboard_repo_service::{ScoreboardRepo, ScoreboardRepoTrait};

pub struct TippingScoreboardService<T = ScoreboardRepo> {
    repo: T,
}

impl<T> TippingScoreboardService<T>
where
    T: ScoreboardRepoTrait,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
    pub async fn entries(&self, year: i32, limit: u32) -> Vec<ScoreboardEntity> {
        self.repo.entries(year, limit).await
    }
}
