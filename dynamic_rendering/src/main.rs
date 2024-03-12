#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
fn main() {
    launch(app)
}

fn app() -> Element {
    rsx! {
        "hello"
        IfElseIs{}
        ImprovingIfElseIs{}
        Clickable{
            children: rsx!{
                button{
                    "Just Button"
                }
            }
        }
        ListRender {}
        ListRenderInline {}
    }
}

struct LoggedIn(bool);
fn IfElseIs() -> Element {
    let mut is = use_signal(|| LoggedIn(false));
    if is.read().0 {
        rsx! {
            button {
                onclick: move |_| is.write().0 = false,
                "logged out"
            }
        }
    } else {
        rsx! {
            button {
                onclick: move |_| is.write().0 = true,
                "login"
            }
        }
    }
}

// 반복되는 코드는 다음과 같이 합칠 수 있음 (이게 더 성능 좋음)

fn ImprovingIfElseIs() -> Element {
    let mut _is = use_signal(|| LoggedIn(false));
    rsx! {
        button{
            onclick: move |_| {
                if _is.read().0 {
                    _is.write().0 = false;
                }else {
                    _is.write().0 = true;
                }
            },
            "{_is.read().0}"
        }
    }
}

//또한 Element는 Option<VNode> 형식을 띄고 있기 때문에, 다음과 같이 할 수도 있다. (값이 None인경우에는 렌더링 하지 않는다. 리액트에서 null을 렌더링 하지 않는 것과 같은 이치)
#[derive(PartialEq, Clone, Props)]
struct ClickableProps {
    children: Element,
}
fn Clickable(props: ClickableProps) -> Element {
    match props.children {
        Some(ch) => {
            rsx! {
                p{"this is a children"},
                {ch}
            }
        }
        _ => {
            rsx! {
                "there is no children"
            }
        }
    }
}

// List와 같은 자료는 Element의 반복자를 통해서 구현할 수 있다.
fn ListRender() -> Element {
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<(String, i32)>::new);

    let comments_lock = comments.read();
    let comments_rendered = comments_lock
        .iter()
        .map(|com| {
            rsx! {
                div{
                    key: "{com.1}",
                    p {"{com.0}"}
                }
            }
        })
        .rev();

    rsx! {
        form {
            onsubmit: move |ev| {
                let commend = format!("{:?}",ev.values().get("commend").unwrap());
                comments.write().push(
                    (commend,next_id())
                );
                next_id += 1;
            },
            input {
                name: "commend"
            }
            input {
                r#type: "submit"
            }
        }
        {comments_rendered}
    }
}

//혹은 다음과 같이 inline으로 구현할 수도 있다.
fn ListRenderInline() -> Element {
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<(String, i32)>::new);
    rsx! {
        form {
            onsubmit: move |ev| {
                let commend = format!("{:?}",ev.values().get("commend").unwrap());
                comments.write().push(
                    (commend,next_id())
                );
                next_id += 1;
            },
            input {
                name: "commend"
            }
            input {
                r#type: "submit"
            }
        }
        for cmd in comments().iter().rev() {
            div{
                key: "{cmd.1}",
                p {"{cmd.0}"}
            }
        }
    }
}
