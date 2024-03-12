#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
fn main() {
    launch(app)
}

fn app() -> Element {
    rsx! {
        "hello"
        MemeEditor{}
    }
}

// 이런식으로 컴포넌트 안의 요소들에게 정보를 전달 할 수 있다.
#[component]
fn Meme(caption: String) -> Element {
    // 상위 컴포넌트에서 제공하는 스텟을 가져옴
    let dark_mode = consume_context::<Signal<DarkMode>>();
    let container_style = r#"
        position: relative;
        width: fit-content;
    "#;

    let caption_containter_style = r#"
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 16px 8px; 
    "#;

    // 상위 컴포넌트에서 제공하는 스텟을 활용하여 다크모드 활성화 여부를 판별함
    let caption_style = if dark_mode().0 {
        r"
        font-size: 32px;
        margin: 0;
        color: dark;
        text-align: center;
    "
    } else {
        r"
        font-size: 32px;
        margin: 0;
        color: white;
        text-align: center;
            
        "
    };

    rsx! {
        div{style: "{container_style}",
            img{src: "https://i.imgflip.com/2zh47r.jpg", height: "500px"}

            div{style: "{caption_containter_style}", p{style:"{caption_style}", "{caption}"}},
        }
    }
}

#[component]
fn CaptionEditor(caption: String, on_input: EventHandler<FormEvent>) -> Element {
    let mut dark_mode = consume_context::<Signal<DarkMode>>();
    let input_style = r"
        border:none;
        background: cornflowerblue;
        padding: 8px 16px;
        margin: 0;  
        border-radius: 4px;
    ";

    let dark_mode_style = if dark_mode().0 {
        r"
        color: dark;
    "
    } else {
        r"
            color:white;
        "
    };

    rsx! {
        input {
            style: "{input_style}{dark_mode_style}",
            value: "{caption}",
            oninput: move |event| on_input.call(event),
            //이런식으로 발생한 이벤트가 다른 컴포넌트에서도 사용되게끔 할 수 있음
        }
        button {
            onclick: move |_| {
                dark_mode.write().0 = !dark_mode().0;
            },

            "darkmode: {dark_mode().0.to_string()}"
        }

    }
}

#[derive(Clone, Copy)]
struct DarkMode(bool);
fn MemeEditor() -> Element {
    // 또한 아래를 통해서 현재 컴포넌트의 하위 컴포넌트들까지 모두 공유하는 하나의 스텟을 만들 수 있음
    use_context_provider(|| Signal::new(DarkMode(true)));
    let mut dark_mod = consume_context::<Signal<DarkMode>>();
    let container_style = r"
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin: 0 auto;
        width: fit-content;
    ";

    let darkmode = if dark_mod().0 {
        r"background-color: dark;"
    } else {
        r""
    };
    let mut caption = use_signal(|| "me waiting for my rust code to compile".to_string());

    rsx! {
        div {
            style: "{container_style}{darkmode}",
            h1{"Meme Editor"}
            Meme{caption: caption}
            CaptionEditor {caption: caption, on_input: move |event: FormEvent| caption.set(event.value())}
        }
    }
}
