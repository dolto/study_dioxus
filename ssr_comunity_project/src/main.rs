#![allow(non_snake_case)]
mod controller;
mod model;
mod viewr;

use axum::{
    routing::{get, post},
    Router,
};
use controller::*;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::Any, opt::auth::Root, sql::Thing, Surreal};
static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

#[derive(Serialize, Deserialize, Debug)]
struct Tt {
    id: Thing,
    lsb: String,
}
#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();
    // set surreal env
    let db_url = std::env::var("DB_URL").expect("you must set on DB_URL in .env file");
    let usr_name = std::env::var("USR_NAME").expect("you must set on USR_NAME in .env file");
    let usr_pass = std::env::var("USR_PASS").expect("you must set on USR_PASS in .env file");

    // println!("{db_url}");
    // println!("{usr_name}");
    // println!("{usr_pass}");
    // connect to surrealDB
    DB.connect(db_url).await.unwrap();
    DB.signin(Root {
        username: usr_name.as_str(),
        password: usr_pass.as_str(),
    })
    .await
    .unwrap();
    DB.use_ns("community_group")
        .use_db("community")
        .await
        .unwrap();

    // set server env
    let server_url = std::env::var("SERVER_URL").expect("you must set on SERVER_URL in .env file");
    let listener = tokio::net::TcpListener::bind(server_url.as_str())
        .await
        .unwrap();

    println!("listening on http://{server_url}");

    let mut posts = DB.query(r#"SELECT id, lsb FROM tt"#).await.unwrap();
    let posts: Vec<Tt> = posts.take(0).unwrap();
    println!("{posts:?}");

    // set axum router
    axum::serve(
        listener,
        Router::new()
            .route("/", get(home_endpoint))
            .route("/post_list/:tags", get(post_list_endpoint))
            .route("/post/:id", get(post_show_endpoint).delete(post_delete))
            .route("/post_set", get(post_set_endpoint).post(post_set))
            .route("/my_page", get(my_page_endpoint))
            .route("/comment_delete/:id", post(comment_delete))
            .into_make_service(),
    )
    .await
    .unwrap();
}
