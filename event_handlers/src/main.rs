#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
fn main() {
    launch(app)
}

fn app() -> Element {
    rsx! {
        button{onclick: move |event| log::info!("Clicked! Event: {event:?}"), "click me!"}

        Propagation{}
        PreventDefault{}
        FancyButton{
            oncustomclick: move |ev| println!("Clicked!")
        }
    }
}

fn Propagation() -> Element {
    rsx! {
        div {
            onclick: move |_ev| {},
            "outer"
            button {
                onclick: move |ev| {
                    ev.stop_propagation(); //만약 이벤트 전파를 막고싶다면 이렇게 하면 됨
                },
                "inner"
            }
        }
    }
}

fn PreventDefault() -> Element {
    rsx! {
        a {
            href: "https://example.com",
            prevent_default: "onclick",
            onclick: |_| log::info!("link clicked"),
            "example.com"
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
