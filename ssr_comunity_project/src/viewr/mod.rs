#![allow(non_snake_case)]

use dioxus::prelude::*;

fn Home() -> Element {
    rsx! {
        h1 {"Hello!"}
        Link {
            "home",
            href:"/"
        }
    }
}
