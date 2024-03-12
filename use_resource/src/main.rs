#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        h1 {"Hello!"}
        UseResourceEx1{}
        UseResourceEx2{}
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

// 아래의 코드처럼 외부 리소스를 가져오기 위해 use_resource를 사용할 수 있다. (단 그에 맞게끔 reqwest와 serde등의 의존성이 필요함)
fn UseResourceEx1() -> Element {
    let mut future = use_resource(move || async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.read() {
        Some(Ok(res)) => rsx! {
            button {onclick: move |_| future.restart(), "Clcick to fetch another doggo"}
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{res.message}"
                }
            }
        },
        Some(Err(_)) => rsx! {div{"Loading dogs failed"}},
        None => rsx! {div{"Loding dogs..."}},
    }
}

#[component]
fn UseResourceEx2() -> Element {
    let mut breed = use_signal(|| "".to_string());
    let mut future = use_resource(move || async move {
        reqwest::get("https://dog.ceo/api/breed/{breed}/images/random")
            // 이런식으로 주소에 hook포함하면 해당하는 hook의 값이 변화 될 때 마다 자동으로 restart를 수행해준다고 함...
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.read() {
        Some(Ok(res)) => rsx! {
            input{
                r#type: "Text",
                value: "{breed}",
                oninput: move |ev| breed.set(ev.value())
            }
            div {
                "{breed}"
                "{res.message}"
            }
        },
        Some(Err(_)) => rsx! {div{"Loading dogs failed"}},
        None => rsx! {div{"Loding dogs..."}},
    }
}
