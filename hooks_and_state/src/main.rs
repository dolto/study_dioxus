#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
fn main() {
    launch(app)
}

// 훅을 이용해서 컴포넌트의 스텟을 설정할 수 있다.
// 훅을 이용하는 규칙은 다음과같다.
// 1. 훅은 컴포넌트 혹은 다른 훅 안에서만 사용할 수 있다.
// 2. 훅은 컴포넌트가 호출될 때 항상 호출되어야 한다. (즉 조건문이나 반복문 안에 있으면 안된다.)
// 3. 훅의 호출순서는 언제나 동일해야한다.
// 4. 훅의 이름은 use_xxx 형식이며, 커스텀 훅을 만들 때도 같은 명명규칙을 따르는 것이 매우 좋다.
fn app() -> Element {
    rsx! {
        button{onclick: move |event| log::info!("Clicked! Event: {event:?}"), "click me!"}

        Propagation{}
        PreventDefault{}
        FancyButton{
            oncustomclick: move |ev| println!("Clicked!")
        }
        VecElement{}
    }
}

fn Propagation() -> Element {
    let mut ev_message = use_signal(|| "".to_string());
    let mut _ev_message = use_signal(|| "".to_string());
    rsx! {
        div {
            onclick: move |_ev| {
                _ev_message.set(format!("{_ev:?}"));
            },
            "outer {_ev_message.read()}"
            button {
                onclick: move |ev| {
                    ev.stop_propagation(); //만약 이벤트 전파를 막고싶다면 이렇게 하면 됨
                    ev_message.set(format!("{ev:?}"));
                },
                "inner {ev_message.read()}"
            }
        }
    }
}

fn PreventDefault() -> Element {
    let mut msg = use_signal(|| "".to_string());
    rsx! {
        a {
            href: "https://example.com",
            prevent_default: "onclick",
            onclick: move |_| msg.set("link clicked".to_string()),
            "example.com {msg.read()}"
        }
    }
}

struct ComplexData(i32);
#[derive(PartialEq, Clone, Props)]
struct FancyButtonProps {
    oncustomclick: EventHandler<ComplexData>, // 커스텀 이벤트의 경우 이름은 언제나 on으로 시작해야한다고 함
                                              // 또한 다양한 데이터를 받을 수 있음
}
fn FancyButton(props: FancyButtonProps) -> Element {
    rsx! {
        button {
            class:"fancy-button",
            onclick: move |ev| props.oncustomclick.call(ComplexData(5)),
            "click me pls"
        }
    }
}

// 기존의 리액트문법에서는 state의 재렌더링되기 위한 값이 바뀐것을 감지하기 위해 레퍼런스를 감지했다.
// 즉 벡터와 같은 자료형은 기존값을 유지하지 못하고, 언제나 복제해야 했었다... 이 이상한 개적화는 좋지 않으므로, 다음과같은 방법을 활용할 수 있게되었다.
fn VecElement() -> Element {
    let mut list = use_signal(Vec::new);

    rsx! {
        p {"list: {list.read():?}"}
        button {
            onclick: move |_| {
                list.with_mut(|li| li.push(li.len()));
            },
            "{list.len()}"
        }
    }
}
