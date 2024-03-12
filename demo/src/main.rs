#![allow(non_snake_case, unused)]
use chrono::Utc;
use dioxus::prelude::*;

mod components;
use components::props_example::*;
fn main() {
    launch(app)
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    let content = "live <b>dangerously</b>";

    rsx! {
        h1{"High-Five counter: {count}"}
        button{onclick: move|_| count += 1, "High Up!"}
        button{onclick: move|_| count -= 1, "High Down!"}

        StoryListing{
            story: StoryItem{
                title: "안녕".to_string(),
                by: "author".to_string(),
                score: count.to_string().parse().unwrap(),
                time: chrono::Utc::now(),
                comments: "comments".to_string()
            }
        }

        hr {}
        img{
            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
            class: "primary_button",
        }

        hr{}
        div {
            dangerous_inner_html: "{content}"
        }
        hr {}
        div {
            p{"and you can use this fmt"},
            p{"this is 7*8 = {7*8}"},
            p{{content}},
            span{
                "map is: "
                {(0..=10).map(|i| rsx!{"{i}"})}
            }
            p{}
            span{
                "for is: "
                for i in 0..=10{
                    "{i}"
                }
            }
        }
        hr {}
        p{"and you can also use the if syntax"}
        if true {
            div {"it is true wow!"}
        }
        hr {}
        p{"and you can use a module in other file"}
        Likes{score:43}

        Title{title: "noSubtitle"}
        Title{title: "subTitleis", subtitle: Some("hooooooooooooh!".to_string())}
        Title{title: "default is", subtitle: None, number: 44}

        IntoComponent{string: "this is &str but props type is String"}

        TitleCard{title: "this is not struct porps".to_string()}

        Clickable {href: "https://www.youtube.com", body: rsx!{"How to " i{"not"} " be seen"}}
        br{}
        ClickableChildren{href: "https://www.youtube.com",
            "This is "
            i{"not"}
            " porps name... it's children field"
        }
    }
}

struct StoryItem {
    title: String,
    by: String,
    score: i32,
    time: chrono::DateTime<Utc>,
    comments: String,
}

#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title,
        by,
        score,
        time,
        comments,
    } = &*story.read();

    rsx! {
        div {
            border: "solid 1px green",
            p {
                "{title} by {by} ({score}) {time} {comments}"
            }
        }
    }
}
