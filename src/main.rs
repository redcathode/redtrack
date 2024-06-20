

use axum::{
    extract::{Query, State}, http::StatusCode, routing::{get, post}, Json, Router
};

use redtrack::{Post, NewPost, FetchedPost};

use serde::Deserialize;

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
        .route("/api/v1/posts/submit", post(submit_posts))
        .route("/api/v1/posts/fetch/all", get(fetch_posts))
        .route("/api/v1/posts/fetch", get(fetch_posts_queried))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn submit_posts(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_posts): Json<Vec<NewPost>>
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let timestamp: i64 = chrono::Utc::now().timestamp();
    let new_posts: Vec<Post> = new_posts.into_iter().map(|x| Post {
        userid: 1, // hardcoded for now
        fieldname: x.fieldname,
        fieldtype: x.fieldtype,
        fieldval: x.fieldval,
        timestamp: timestamp
    }).collect();
    // Insert the new post into the database
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn.interact(move |conn| {
        new_posts.iter().map(|x| {
            redtrack::add_post_to_db(conn, x)
        }).collect::<Vec<Post>>()
    }).await
    .map_err(internal_error)?;

    Ok(Json(res))
}

async fn fetch_posts(
    State(pool): State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<FetchedPost>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn.interact(|conn| {
        redtrack::schema::posts::table
            .load::<FetchedPost>(conn)
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct FetchPostsQuery {
    begin_ts: Option<i64>,
    end_ts: Option<i64>,
    fieldname: Option<String>
}

async fn fetch_posts_queried(
    Query(fetch_query): Query<FetchPostsQuery>,
    State(pool): State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<FetchedPost>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    // TODO: once multiuser implemented, fix this
    let res = conn.interact(move |conn| {
        let mut query = redtrack::schema::posts::table.into_boxed();
        if let Some(begin_ts) = fetch_query.begin_ts {
            query = query.filter(redtrack::schema::posts::timestamp.ge(begin_ts));
        }
        if let Some(end_ts) = fetch_query.end_ts {
            query = query.filter(redtrack::schema::posts::timestamp.le(end_ts));
        }
        if let Some(fieldname) = fetch_query.fieldname { // TODO: multiple fieldnames in one request
            query = query.filter(redtrack::schema::posts::fieldname.eq(fieldname));
        }
        query.load::<FetchedPost>(conn)
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