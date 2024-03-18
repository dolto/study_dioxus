use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Thing,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: Thing,
    pub user_id: String,
    pub title: String,
    pub created_at: Datetime,
    pub value: String,
    pub tag: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: Thing,
    pub perrent_id: Option<Id>,
    pub value: String,
    pub created_at: Datetime,
}
