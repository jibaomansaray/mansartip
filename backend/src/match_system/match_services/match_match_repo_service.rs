use crate::{
    app::app_state::{DbPool, DbRow},
    country_system::country_entities::CountryEntity,
    match_system::match_entities::match_entity::MatchEntity,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use std::sync::Arc;

#[async_trait]
pub trait MatchRepoServiceTrait {
    async fn todays(&self, year: i32) -> Vec<MatchEntity>;
    async fn all(&self, year: i32) -> Vec<MatchEntity>;
}

pub struct MatchRepoService {
    pool: Arc<DbPool>,
}

impl MatchRepoService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
    fn buil_sql(where_clause: &str) -> (String, impl Fn(DbRow) -> MatchEntity) {
        let template =
            "SELECT `match`.*, {country_a_fields}, {country_b_fields}, {winner_fields} FROM `match`
         LEFT JOIN `country` as `country_a` on `match`.`countryAId` = `country_a`.`id` 
         LEFT JOIN `country` as `country_b` on `match`.`countryBId` = `country_b`.`id` 
         LEFT JOIN `country` as `winner` on `match`.`winnerId` = `winner`.`id`";

        let country_a_fields = CountryEntity::generate_join_fields(Some("country_a"));
        let country_b_fields = CountryEntity::generate_join_fields(Some("country_b"));
        let winner_fields = CountryEntity::generate_join_fields(Some("winner"));

        let template = template
            .replace("{country_a_fields}", &country_a_fields.fields)
            .replace("{country_b_fields}", &country_b_fields.fields)
            .replace("{winner_fields}", &winner_fields.fields);

        (format!("{} WHERE {}", template, where_clause), move |row| {
            let mut entity = MatchEntity::from_row_ref(&row);
            entity.country_a = country_a_fields.transform(&row);
            entity.country_b = country_b_fields.transform(&row);
            entity.winner = winner_fields.transform(&row);

            entity
        })
    }
}

#[async_trait]
impl MatchRepoServiceTrait for MatchRepoService {
    async fn todays(&self, year: i32) -> Vec<MatchEntity> {
        let _condition= "`match`.`status` = 'open' AND year(`match`.`date`) = ? AND `match`.`date` <= now()  AND  `match`.`time` >= now();";

        // for testing only
        let condition = "`match`.`status` = 'open' AND year(`match`.`date`) = ?";

        let built = Self::buil_sql(condition);
        let mut result = Vec::new();

        let mut rows = sqlx::query(&built.0)
            .bind(year)
            .map(built.1)
            .fetch(self.pool.as_ref());

        while let Some(entity) = rows.try_next().await.expect("could not get matches") {
            result.push(entity);
        }

        result
    }

    async fn all(&self, year: i32) -> Vec<MatchEntity> {
        let mut result = Vec::new();
        let built = Self::buil_sql("year(`match`.`date`) = ?");

        let mut rows = sqlx::query(&built.0)
            .bind(year)
            .map(built.1)
            .fetch(self.pool.as_ref());

        while let Some(entity) = rows.try_next().await.expect("could not get matches") {
            result.push(entity);
        }

        result
    }
}

/*
       {
           "id": 1,
           "status": "open",
           "year": 2023,
           "number": 1,
           "date": "2023-11-20",
           "time": "16:00:00",
           "round": "group",
           "match": "A1 v. A2",
           "penalty": false,
           "countryAGoals": 0,
           "countryBGoals": 0,
           "countryAPenaltyGoals": 0,
           "countryBPenaltyGoals": 0,
           "toConfigureOn": null,
           "tips": [],
           "countryA": {
               "imageSource": "/static/flag/QAT_qatar.png",
               "id": 1,
               "internalId": "A1",
               "year": 2023,
               "name": "Qatar",
               "short": "QAT",
               "groupPoints": -1,
               "image": "QAT_qatar.png",
               "deletedAt": null
           },
           "countryB": {
               "imageSource": "/static/flag/ECU_ecuador.png",
               "id": 2,
               "internalId": "A2",
               "year": 2023,
               "name": "Ecuador",
               "short": "ECU",
               "groupPoints": -1,
               "image": "ECU_ecuador.png",
               "deletedAt": null
           },
           "winner": null,
           "tip": {
               "id": 0,
               "year": 2023,
               "countryAToScore": 0,
               "countryBToScore": 0,
               "countryAPenaltyToScore": 0,
               "countryBPenaltyToScore": 0,
               "isLevel": false,
               "toPenalty": false,
               "points": 0,
               "entryByBot": false
           }
       },
*/
