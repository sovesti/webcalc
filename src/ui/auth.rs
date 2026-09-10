use dioxus::prelude::*;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        input::Input,
        tabs::{TabContent, TabList, TabTrigger, Tabs},
    },
    user::api::sign_in,
};

#[component]
pub fn AuthView(authorized: Signal<bool>) -> Element {
    rsx! {
        Tabs {
            default_value: "sign_in".to_owned(),
            class: "dx-tabs",
            TabList {
                class: "dx-tab-list",
                TabTrigger {
                    class: "dx-tab-trigger",
                    index: 0usize,
                    value: "sign_in",
                    "Sign in"
                },
                TabTrigger {
                    class: "dx-tab-trigger",
                    index: 1usize,
                    value: "sign_up",
                    "Sign up"
                }
            },
            TabContent {
                index: 0usize,
                value: "sign_in",
                SignInForm { authorized }
            },
            TabContent {
                index: 1usize,
                value: "sign_up",
                SignUpForm { authorized }
            }
        }
    }
}

#[component]
fn SignInForm(authorized: Signal<bool>) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut signin = use_action(move || try_sign_in(username, password, authorized));
    rsx! {
        div {
            class: "flex-1 flex-col",
            Input {
                class: "dx-input",
                oninput: move |e: FormEvent| username.set(e.value()),
                placeholder: "Name",
                value: username
            },
            Input {
                class: "dx-input",
                oninput: move |e: FormEvent| password.set(e.value()),
                placeholder: "Password",
                type: "password",
                value: password
            },
            Button {
                class: "dx-button",
                variant: ButtonVariant::Outline,
                onclick: move |_| signin.call(),
                "Sign in"
            },
            if let Some(Err(err)) = signin.value() {
                p { "{err}" }
            },
            if signin.pending() {
                p { "Checking..." }
            }
        }
    }
}

async fn try_sign_in(
    username: Signal<String>,
    password: Signal<String>,
    mut authorized: Signal<bool>,
) -> Result<()> {
    sign_in(username.to_string(), password.to_string()).await?;
    authorized.set(true);
    Ok(())
}

#[component]
fn SignUpForm(authorized: Signal<bool>) -> Element {
    rsx! {
        p {
            "TODO: Sign up"
        }
    }
}
