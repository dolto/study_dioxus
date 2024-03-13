#![allow(non_snake_case)]
use dioxus::prelude::*;
fn main() {
    launch(App)
}

fn App() -> Element {
    let mut count = use_signal(|| 0);
    let message = use_signal(|| "".to_string());
    // 서버로부터 데이터 가져오기
    let elary_message = use_resource(get_server_message);
    rsx! {
        h1 {"counter: {count}"}
        //클라이언트에서 처리
        button {onclick: move |_| count += 1, "Up"}
        button {onclick: move |_| count -= 1, "Down"}
        //서버에서 처리
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

        // 서버로부터 데이터 가져오기
        p{"message: {message}"}
        p{"elary_message: {elary_message.value():?}"}
        button {
            onclick: move |_| {
                to_owned![message];
                async move {
                    if let Ok(new_message) = get_server_message().await{
                        message.set(new_message);
                    }
                }
            },
            "get server message"
        }
    }
}

#[server]
async fn double_server(number: i32) -> Result<i32, ServerFnError> {
    let result = number * 2;
    println!("server result is: {result}");

    Ok(result)
}

#[server]
async fn get_server_message() -> Result<String, ServerFnError> {
    Ok("Hello message from server!".to_string())
}
