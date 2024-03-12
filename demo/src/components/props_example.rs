use dioxus::prelude::*;

// 프롭을 가진 컴포넌트는 다음과 같이 구현이 가능하다.
#[derive(PartialEq, Props, Clone)]
pub struct LikeProps {
    score: i32,
}

pub fn Likes(props: LikeProps) -> Element {
    rsx! {
        div {
            "스코어: "
            b {"{props.score}"}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct OptionalProps {
    title: String,
    #[props(optional)] // 생략 가능 혹은 !optional을 사용해서 반드시 값을 넣게 강제 할수 있음
    subtitle: Option<String>,
    #[props(default = 33)]
    number: i32,
}

pub fn Title(props: OptionalProps) -> Element {
    rsx! {
        h1 {
            "{props.title}: ", {props.subtitle.unwrap_or_else(|| "No subtitle provide".to_string())}
        }
        h3 {"{props.number}"}
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct IntoProps {
    #[props(into)] //이렇게 하면 into()가 되는 형태의 모든 타입을 받을 수 있음
    string: String,
}

pub fn IntoComponent(props: IntoProps) -> Element {
    rsx! {
        h5 {
            {props.string}
        }
    }
}

// 단순한 변수를 활용하는 프롭이라면 다음과같이 활용할 수도 있으나, 권장되는 사항은 아니다.
#[component]
pub fn TitleCard(title: String) -> Element {
    rsx! {
        h2 {"{title}"}
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ClickableProps {
    href: String,
    body: Element,
}

pub fn Clickable(props: ClickableProps) -> Element {
    rsx! {
        a{ href: "{props.href}", class: "fancy-button", {props.body}}
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ClickablechildreneProps {
    href: String,
    children: Element,
}

pub fn ClickableChildren(props: ClickablechildreneProps) -> Element {
    rsx! {
        a{ href: "{props.href}", class: "fancy-button", {props.children}}
    }
}
