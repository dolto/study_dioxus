#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
fn main() {
    launch(app)
}

// 사용자 입력은 다음과 같이 제어형식으로 구현하는 방법이 있다.
// 이렇게하면 실시간으로 입력을 감지할 수 있으므로 입력 받을 때 마다 필요한 기능들을 수행하게 할 수 있다. (예: 자동완성, 대/소문자, 유효성 검사 등)

fn app() -> Element {
    let mut name = use_signal(|| "bob".to_string());
    rsx! {
        p {
            "{name}"
        }
        input {
            value: "{name}",
            oninput: move |ev| name.set(ev.value())
        }

        UncontrolledInput {}

        hr{}

        FileHandleing{}
    }
}

// 그러나 제어된 방식의 입력형식은 성능을 많이 잡아먹을 수 있기 때문에, 그에대한 비제어형식의 입력형식이 존재한다.
// 그러나 HTML의 내장기능으로 어느정도의 포멧 제어정도는 가능하니, 그점을 적극 이용하면 괜찮은 대안이 될 수 있다.

fn UncontrolledInput() -> Element {
    let mut log = use_signal(|| "log: ".to_string());

    rsx! {
        p{"{log.read()}"}
        form {
            prevent_default: "onsubmit",
            onsubmit: move |ev| log.set(format!("log: {ev:?}")),
            input {name: "name"},
            input {name: "age", r#type:"number"},
            input {name: "date", r#type:"date"},
            input {r#type: "submit"}
        }
    }
}

fn FileHandleing() -> Element {
    let mut filenames = use_signal(Vec::new);
    let mut files_uploaded = use_signal(Vec::new);
    let mut dir = use_signal(Vec::new);
    rsx! {
        p {"파일 이름 리스트: {filenames:?}"}
        input {
            r#type: "file",
            accept: ".txt, .rs",
            multiple: true,
            onchange: move |ev| {
                if let Some(file_engine) = &ev.files() {
                    let files = file_engine.files();
                    for name in files {
                        filenames.write().push(name);
                    }
                }
            }
        }

        p {"파일 내용 리스트: {files_uploaded:?}"}
        input {
            r#type: "file",
            accept: ".txt, .rs, .ts, .java",
            onchange: move |ev| {
                to_owned![files_uploaded];
                async move {
                    if let Some(file_engine) = ev.files() {
                        let files = file_engine.files();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file_to_string(file_name).await {
                                files_uploaded.write().push(file);
                            }
                        }
                    }
                }
            }
        }

        p{"디렉토리 리스트: {dir:?}"}
        input {
            r#type: "file",
            directory: true,
            onchange: move |ev| {
                if let Some(file_engine) = ev.files() {
                    let files = file_engine.files();
                    for name in files {
                        dir.write().push(name);
                    }
                }
            }
        }

    }
}
