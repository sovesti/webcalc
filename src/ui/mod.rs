pub(crate) mod auth;
pub(crate) mod eval;

use dioxus::prelude::*;

use crate::ui::{auth::AuthView, eval::EvalView};

pub fn app() -> Element {
    let authorized = use_signal(|| false);
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") },
        Stylesheet { href: asset!("/assets/tailwind.css") },
        main {
            div {
                class: "min-h-screen p-4",
                if authorized() {
                    EvalView {},
                } else {
                    AuthView { authorized }
                }
            }
        }
    }
}
