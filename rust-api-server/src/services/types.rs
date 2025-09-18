use diesel::prelude::*;
use diesel::sql_types::{Array, BigInt, Double, Integer};

#[derive(QueryableByName, Debug)]
pub struct Feature2Entity {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Array<Double>"]
    pub feature: Vec<f64>,
    #[sql_type = "Integer"]
    pub distance: i32,
}

#[derive(QueryableByName, Debug)]
pub struct Feature1Entity {
    #[sql_type = "Array<Double>"]
    pub feature: Vec<f64>,
}
