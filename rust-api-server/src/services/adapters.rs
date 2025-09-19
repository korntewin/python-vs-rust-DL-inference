use crate::config::Config;
use crate::services::types::{Feature1Entity, Feature2Entity};
use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{Array, BigInt, Double, Integer};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[async_trait]
pub trait GetFeaturePort: Send + Sync {
    async fn query_feature_1(&self, feature_1_id: i64) -> Result<Vec<f32>>;

    async fn query_feature_2(
        &self,
        feature_2_ids: Vec<i64>,
        latitude: f64,
        longitude: f64,
        max_dist: i32,
    ) -> Result<(Vec<i64>, Vec<Vec<f32>>, Vec<i32>)>;
}

type PgPool = Pool<AsyncPgConnection>;

#[derive(Clone)]
pub struct GetFeatureFromSQLAdapter {
    pool: PgPool,
}

impl GetFeatureFromSQLAdapter {
    pub async fn new(config: Config) -> Result<Self> {
        let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.db_url);
        let pool = Pool::builder(mgr).build()?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl GetFeaturePort for GetFeatureFromSQLAdapter {
    async fn query_feature_2(
        &self,
        feature_2_ids: Vec<i64>,
        latitude: f64,
        longitude: f64,
        max_dist: i32,
    ) -> Result<(Vec<i64>, Vec<Vec<f32>>, Vec<i32>)> {
        let mut conn = self.pool.get().await?;

        const SQL: &str = r#"
WITH features AS (
    SELECT 
        id,
        feature,
        ST_Distance(
            geog,
            ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography
        )::integer AS distance
    FROM feature_2
    WHERE id = ANY($3)
)
SELECT id, feature, distance
FROM features
WHERE distance < $4
"#;
        let rows = diesel::dsl::sql_query(SQL)
            .bind::<Double, _>(longitude)
            .bind::<Double, _>(latitude)
            .bind::<Array<BigInt>, _>(feature_2_ids)
            .bind::<Integer, _>(max_dist)
            .load::<Feature2Entity>(&mut conn)
            .await?;
        Ok((
            rows.iter().map(|r| r.id).collect(),
            rows.iter()
                .map(|r| r.feature.iter().map(|&f| f as f32).collect())
                .collect(),
            rows.iter().map(|r| r.distance).collect(),
        ))
    }

    async fn query_feature_1(&self, user_id: i64) -> Result<Vec<f32>> {
        let mut conn = self.pool.get().await?;
        let user_feature = diesel::dsl::sql_query(
            r#"
            SELECT feature
            FROM feature_1
            WHERE id = $1
        "#,
        )
        .bind::<BigInt, _>(user_id)
        .get_result::<Feature1Entity>(&mut conn)
        .await?;
        Ok(user_feature.feature.iter().map(|&f| f as f32).collect())
    }
}
