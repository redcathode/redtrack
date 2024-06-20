use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub userid: i64,
    pub fieldname: String,
    pub fieldtype: String,
    pub fieldval: Option<String>,
    pub timestamp: i64
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FetchedPost {
    pub id: i32,
    pub userid: i64,
    pub fieldname: String,
    pub fieldtype: String,
    pub fieldval: Option<String>,
    pub timestamp: i64

}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub fieldname: String,
    pub fieldtype: String,
    pub fieldval: Option<String>
}
