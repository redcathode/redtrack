use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub notes: Option<String>,
    pub overall: Option<f64>,
    pub psychomotor: Option<f64>,
    pub energy: Option<f64>,
    pub mood: Option<f64>,
    pub thoughts_slowed_racing: Option<f64>,
    pub concentration_difficulty: Option<f64>,
    pub time_submitted: i64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub notes: Option<String>,
    pub overall: Option<f64>,
    pub psychomotor: Option<f64>,
    pub energy: Option<f64>,
    pub mood: Option<f64>,
    pub thoughts_slowed_racing: Option<f64>,
    pub concentration_difficulty: Option<f64>,
    pub time_submitted: Option<i64>,
}