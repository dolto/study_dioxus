#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home{},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList{},
                #[route("/post/:name")]
                BlogPost {name: String},
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[nest("/myblog")] // myblog를 기준으로 접속을 시도해도 정상적인 경로로 리디렉션이 이루어짐
        #[redirect("/", || Route::BlogList{})]
        #[redirect("/:name", |name: String| Route::BlogPost{name})]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
    launch(App)
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            ul{
                li{
                    Link {to: Route::Home {}, "Home"}
                }
                li {
                    Link {to: Route::BlogList {}, "Blog"}
                }
            }
        }
        Outlet::<Route> {} // 하위 경로에 있는 부분 렌더링을 위한 키워드
    }
}

#[component]
fn Home() -> Element {
    rsx! {h1 {"Welcome to the Dioxus Blog!"}}
}

#[component]
fn Blog() -> Element {
    rsx! {
        h1 {"Blog"}
        Outlet::<Route> {}
    }
}

#[component]
fn BlogList() -> Element {
    rsx! {
        h2 {"Choose a post"}
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the scend blog post"
                }
            }
        }
    }
}

#[component]
fn BlogPost(name: String) -> Element {
    rsx! {h2 {"Blog Post: {name}"}}
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1{"Page not found"}
        p {"We are terribly srry, but the page you requested doesn't exist."}
        pre {color: "red", "log:\nattemped to navigate to: {route:?}"}
    }
}
