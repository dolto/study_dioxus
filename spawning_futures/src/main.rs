#![allow(non_snake_case, unused)]
use std::time::Duration;

use dioxus::prelude::*;

fn main() {
    launch(app)
}

// dioxus에서 제공하는 비동기 처리는 다음과 같이 하면 된다고 하는데
// ex2는 수행되지도 않고, 코드가 먹통이 되어버림...
fn app() -> Element {
    rsx! {
        h1 {"Hello"}
        SpawningFutersEx1{}
        SpawningFutersEx2{}
    }
}

fn SpawningFutersEx1() -> Element {
    let res = use_signal(|| String::from("..."));

    let log_in = move |_| {
        spawn({
            to_owned![res];
            async move {
                let resp = reqwest::Client::new()
                    .get("https://dioxuslabs.com")
                    .send()
                    .await;

                match resp {
                    Ok(_data) => {
                        res.set("dioxuslabs.com responded!".into());
                    }
                    Err(err) => res.set("Request failed with error: {err:?}".into()),
                }
            }
        });
    };
    rsx! {
        button{
            onclick: log_in, "Response: {res}"
        }
    }
}

fn SpawningFutersEx2() -> Element {
    let mut sec = use_signal(|| 0);

    let timer = move |_| {
        spawn({
            to_owned![sec];
            async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    sec += 1;
                }
            }
        });
    };

    rsx! {
        button {
            onclick: timer, "Timer Start!"
        }
        h3 {"{sec}"}
    }
}
