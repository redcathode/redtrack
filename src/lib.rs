pub mod models;
pub mod schema;

pub use crate::models::Post;
pub use crate::models::NewPost;
use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn add_post_to_db(conn: &mut PgConnection, new_post: NewPost) -> Post {
    // Insert the new post into the database
    let new_post = NewPost {
        notes: new_post.notes,
        overall: new_post.overall,
        psychomotor: new_post.psychomotor,
        energy: new_post.energy,
        mood: new_post.mood,
        thoughts_slowed_racing: new_post.thoughts_slowed_racing,
        concentration_difficulty: new_post.concentration_difficulty,
        time_submitted: Some(chrono::Utc::now().timestamp()),
    };

    diesel::insert_into(schema::posts::table)
        .values(new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}