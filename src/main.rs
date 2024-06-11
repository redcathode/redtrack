

use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};

use redtrack::models::Post;
use redtrack::NewPost;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager).build().unwrap();

    let app = Router::new()
        .route("/posts/submit", post(submit_post))
        .route("/posts", get(view_posts))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn submit_post(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_post): Json<NewPost>
) -> Result<Json<Post>, (StatusCode, String)> {
    // Insert the new post into the database
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn.interact(|conn| {
        redtrack::add_post_to_db(conn, new_post)
    }).await
    .map_err(internal_error)?;

    Ok(Json(res))
}

async fn view_posts(
    State(pool): State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn.interact(|conn| {
        redtrack::schema::posts::table
            .load::<Post>(conn)
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}