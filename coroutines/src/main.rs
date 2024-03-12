#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use futures_util::StreamExt;
use std::time::Duration;

//서버가 필요한건지 뭔진 모르겠지만 도큐먼트로만은 이해할 수 없는 주제이기 때문에 일단 패스

fn main() {
    launch(app)
}

fn app() -> Element {
    let mut sec = use_signal(|| 0);
    let mut sync_task = move || {
        to_owned![sec];
        use_coroutine(|rx: UnboundedReceiver<i32>| async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                sec += 1;
            }
        });
    };

    rsx! {
        button {
            onclick: move |_| sync_task(),
            "time start"
        }
        h1{"{sec}"}
    }
}

enum ProfileUpdate {
    SetUsername(String),
    SetAge(i32),
}
