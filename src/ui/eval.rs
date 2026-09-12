use dioxus::prelude::*;

use crate::{
    eval::api::eval_expr,
    user::api::user_name,
};

#[component]
pub fn EvalView() -> Element {
    let name = use_resource(user_name);

    let mut expression = use_signal(String::new);
    let mut evaluate = use_action(move || try_eval(expression));

    rsx! {
        div {
            class: "flex flex-col",

            match &*name.read_unchecked() {
                Some(Ok(user)) => rsx! { p { "Hello, {user}" } },
                Some(Err(err)) => rsx! { p { "Error: {err}" } },
                None => rsx! { p { "Loading..." } },
            }

            input {
                class: "flex-1 m-1 p-2",
                placeholder: "Expression",
                value: expression,
                oninput: move |e: FormEvent| expression.set(e.value()),
            }

            button {
                class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                onclick: move |_| evaluate.call(),
                "Evaluate"
            }

            if evaluate.pending() {
                p { "Calculating..." }
            }

            if let Some(Err(err)) = evaluate.value() {
                p { "Error: {err}" }
            }

            if let Some(Ok(_)) = evaluate.value() {
                p { "Done" }
            }
        }
    }
}

async fn try_eval(expression: Signal<String>) -> Result<()> {
    eval_expr(expression.to_string()).await?;
    Ok(())
}