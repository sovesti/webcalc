use dioxus::prelude::*;

use crate::user::api::{sign_in, sign_up};

#[derive(PartialEq)]
enum Tab {
    SignIn,
    SignUp,
}

#[component]
pub fn AuthView(authorized: Signal<bool>) -> Element {
    let mut tab = use_signal(|| Tab::SignIn);
    rsx! {
        div {
            class: "flex flex-col",
            div {
                class: "flex flex-row",
                button {
                    class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                    class: if *tab.read() == Tab::SignIn { "bg-slate-100" },
                    onclick: move |_| tab.set(Tab::SignIn),
                    "Sign in"
                },
                button {
                    class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                    class: if *tab.read() == Tab::SignUp { "bg-slate-100" },
                    onclick: move |_| tab.set(Tab::SignUp),
                    "Sign up"
                }
            }
            match *tab.read() {
                Tab::SignIn => rsx! { SignInForm { authorized } },
                Tab::SignUp => rsx! { SignUpForm { authorized } },
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
            class: "flex flex-col",
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| username.set(e.value()),
                placeholder: "Name",
                value: username
            },
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| password.set(e.value()),
                placeholder: "Password",
                type: "password",
                value: password
            },
            button {
                class: "rounded-xl m-1 p-2 hover:bg-slate-200",
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
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut signup = use_action(move || try_sign_up(username, password, authorized));
    rsx! {
        div {
            class: "flex flex-col",
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| username.set(e.value()),
                placeholder: "Name",
                value: username
            },
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| password.set(e.value()),
                placeholder: "Password",
                type: "password",
                value: password
            },
            button {
                class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                onclick: move |_| signup.call(),
                "Sign up"
            },
            if let Some(Err(err)) = signup.value() {
                p { "{err}" }
            },
            if signup.pending() {
                p { "Registering..." }
            }
        }
    }
}

async fn try_sign_up(
    username: Signal<String>,
    password: Signal<String>,
    authorized: Signal<bool>,
) -> Result<()> {
    sign_up(username.to_string(), password.to_string()).await?;
    try_sign_in(username, password, authorized).await
}
