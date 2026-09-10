pub(crate) mod auth;
pub(crate) mod eval;

use dioxus::prelude::*;

use crate::ui::{auth::AuthView, eval::EvalView};

pub fn app() -> Element {
    let authorized = use_signal(|| false);
    rsx! {
        if authorized() {
            EvalView {},
        } else {
            AuthView { authorized }
        }
    }
}
