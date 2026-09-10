use dioxus::prelude::*;

use crate::user::api::user_name;

#[component]
pub fn EvalView() -> Element {
    let name = use_resource(user_name);
    rsx! {
        match &*name.read_unchecked() {
            Some(Ok(user)) => rsx! { p { "Hello, {user}" } },
            Some(Err(err)) => rsx! { p { "Error: {err}" } },
            None => rsx! { p { "Loading..." } },
        }
    }
}
