pub mod models;
pub mod schema;

pub use crate::models::{Post, NewPost, FetchedPost};
use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn add_post_to_db(conn: &mut PgConnection, new_post: &Post) -> Post {
    diesel::insert_into(schema::posts::table)
        .values(new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}