#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::controller::PostId;

pub fn Home() -> Element {
    rsx! {
        h1 {"Hello!"}
        a {
            href: "/post_list/",
            "get post list",
        }
    }
}
pub fn PostList(posts: Vec<PostId>) -> Element {
    let mut tags: Signal<Vec<String>> = use_signal(|| Vec::new());
    let mut input_tag = use_signal(|| "".to_string());

    let result = tags().join(",");
    rsx! {
        h1{"Post List"}
        input{value: "{input_tag}", oninput: move |e| input_tag.set(e.value())}
        button{
            onclick: move |_| {
                tags.with_mut(|t| t.push(input_tag().clone()));
                input_tag.set("".to_string());
            },
            "| + |"
        }
        for tag in tags(){
            p{
                key: "{tag}",
                onclick: move |_| {
                    tags.with_mut(|t| t.retain(|t| t.as_str() != tag.as_str()));
                },
                "{tag}",
            }
        }
        a{
            href: "/post_list/{result}",
            "submit",
        }
        hr{}
        for post in posts{
            p{
                key: "{post.id:?}",
                a{
                    href: "/post/{post.id.id.to_string()}",
                    "name: {post.title}, created_at {post.created_at}"
                }
            }
        }
    }
}
