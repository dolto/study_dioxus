#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

fn main() {
    launch(App)
}

fn App() -> Element {
    let mut count = use_signal(|| 0);
    let count2 = use_resource(get_server_data);
    let count3 = use_signal(|| Vec::new());
    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button {
            onclick: move |_| {
                to_owned![count];
                async move {
                    if let Ok(new_count) = double_server(count()).await {
                        count.set(new_count);
                    }
                }
            },
            "Double"
        }
        hr{}
        h1{"Count2 is {count2.value():?}"}
        hr{}
        h1{"Count3 for Vec is {count3.read():?}"}
        button {
            onclick: move |_|{
                to_owned![count, count3];
                async move {
                    if let Ok(new_vec) = get_surreal_db(count()).await{
                        count3.set(new_vec);
                    }
                }
            },
            "save"
        }
    }
}

#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let result = number * 2;
    println!("server calculated {result}");
    Ok(result)
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Hello from the server".to_string())
}

// need docker run --rm --pull always -p 80:8000 surrealdb/surrealdb:latest start -A memory --user root --pass root

#[derive(Serialize, Deserialize)]
struct Count {
    value: i32,
}
#[server]
async fn get_surreal_db(number: i32) -> Result<Vec<i32>, ServerFnError> {
    println!("try connect surrealDB");
    let db = Surreal::new::<Ws>("localhost:80").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    println!("try ns surrealDB");
    db.use_ns("counter").use_db("db").await?;
    println!("try create surrealDB");
    let _: Vec<Count> = db
        .create("counter")
        .content(Count { value: number })
        .await?;
    println!("try select surrealDB");
    let selected: Vec<Count> = db.select("counter").await?;
    let result = selected.iter().map(|c| c.value).collect();
    println!("{result:?}");
    Ok(result)
}
