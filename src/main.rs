#![allow(non_snake_case)]
mod state;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use state::AppState;

fn main() {
    launch(App);
}



fn App() -> Element {
    use_context_provider(AppState::new);

    rsx! { h1 { "Hello World" } }
}

#[component]
fn Loader() -> Element {
    rsx! {
        div { class: "flex justify-center items-center",
            div { class: "animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-purple-500" }
        }
    }
}

fn About() -> Element {
    rsx! {
        div { class: "flex flex-col items-center",
            p {
                "You are looking at a "
                span { class: "font-bold", "Actualism" }
                " app (see source code "
                ExternalLink {
                    href: "https://github.com/srid/actualism-app",
                    title: "Github repository",
                    "here"
                }
                ")"
            }
            a { href: "https://dioxuslabs.com/", img { class: "w-32 h-32", src: "dioxus.png" } }
        }
    }
}

#[component]
fn ExternalLink(href: &'static str, title: &'static str, children: Element) -> Element {
    rsx! {
        a {
            class: "text-purple-600 hover:text-purple-800",
            href: "{href}",
            title: "{title}",
            {children}
        }
    }
}
