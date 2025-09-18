use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub feature_2_ids: Vec<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub size: i32,
    pub max_dist: i32,
    pub sort_dist: bool,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub id: i64,
    pub score: f64,
    pub displacement: i32,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub outputs: Vec<Output>,
}
