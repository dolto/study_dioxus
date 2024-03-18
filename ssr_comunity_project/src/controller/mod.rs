use axum::{
    extract::Path,
    response::{Html, Redirect},
    Json,
};
use chrono::Utc;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use crate::{model::Post, DB};

// .route("/", get(home_endpoint))
// .route("/post_list", get(post_list_endpoint))
// .route(
//     "/post_list/:id",
//     get(post_show_endpoint).delete(post_delete),
// )
// .route("/post_set/:id", get(post_set_endpoint).post(post_set))
// .route("/my_page", get(my_page_endpoint))
// .route("/comment_delete/:id", post(comment_delete))
// .into_make_service(),
pub async fn home_endpoint() -> Html<String> {
    Html(dioxus_ssr::render_element(rsx! {"Hello"}))
}

#[derive(Deserialize, Debug)]
pub struct PostListSearchTags {
    value: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
struct PostId {
    id: Thing,
    created_at: Datetime,
    title: String,
    tag: Vec<String>,
}
pub async fn post_list_endpoint(Path(tags): Path<String>) -> Html<String> {
    let tags: Vec<String> = tags.split(',').map(|t| t.to_string()).collect();
    println!("{tags:?}");
    let mut posts = DB
        .query("SELECT id, tag, created_at, title FROM posts")
        .await
        .unwrap();
    let posts: Vec<PostId> = posts.take(0).unwrap_or(vec![]);
    // let test_post: Vec<Post> = DB.select("posts").await.unwrap();
    // println!("{test_post:?}");
    println!("{posts:?}");
    let posts = posts
        .iter()
        .filter(|p| {
            for tag in tags.iter() {
                if p.tag.contains(tag) {
                    return true;
                }
            }
            false
        })
        .map(|p| {
            rsx! {
                p{
                    "{p.title}, {p.created_at.to_string()}"
                }
            }
        });
    Html(dioxus_ssr::render_element(rsx! {{posts}}))
}

// #[derive(Deserialize)]
// struct Id {
//     id: String,
// }
pub async fn post_show_endpoint(Path(id): Path<String>) -> Html<String> {
    println!("{id}");
    let post: Option<Post> = DB.select(("posts", id.as_str())).await.unwrap();
    println!("{post:?}");
    let post = post.unwrap();
    Html(dioxus_ssr::render_element(rsx! {
        h1{"{post.title}"}
        div{"{post.value}"}
        p{"{post.created_at.to_string()}"}
        p{"post.tags:?"}
    }))
}

pub async fn post_delete(Path(id): Path<String>) -> Redirect {
    let _: Option<Post> = DB.delete(("posts", id.as_str())).await.unwrap();
    Redirect::permanent("/")
}

pub async fn post_set_endpoint(Path(id): Path<String>) -> Html<String> {
    let post: Option<Post> = DB.select(("posts", id.as_str())).await.unwrap();
    let post = post.unwrap();
    Html(dioxus_ssr::render_element(rsx! {
        h1{"{post.title}"}
        div{"{post.value}"}
        p{"{post.created_at.to_string()}"}
        p{"post.tags:?"}
    }))
}

pub async fn post_set(Path(id): Path<String>, Json(post): Json<Post>) -> Redirect {
    let post_id;
    if id.len() > 0 {
        post_id = id.clone();
        let _: Option<Post> = DB
            .update(("posts", id.as_str()))
            .content(post)
            .await
            .unwrap();
    } else {
        let post: Vec<Post> = DB.create("posts").content(post).await.unwrap();
        post_id = post[0].id.clone().to_string();
    }
    Redirect::permanent(format!("/post_list/{post_id}").as_str())
}

pub async fn my_page_endpoint() -> Html<String> {
    Html(dioxus_ssr::render_element(rsx! {
        "제작중..."
    }))
}
pub async fn comment_delete() -> Html<String> {
    Html(dioxus_ssr::render_element(rsx! {
        "제작중..."
    }))
}
