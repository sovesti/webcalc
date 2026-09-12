use dioxus::prelude::*;

use crate::{
    eval::{
        EvaluatedExpression,
        api::{eval_expr, eval_history},
    },
    user::api::user_name,
};

#[component]
pub fn EvalView() -> Element {
    let name = use_resource(user_name);
    rsx! {
        div {
            class: "flex flex-col",
            CurrentUser { name },
            ExpressionView {},
            HistoryView {}
        }
    }
}

#[component]
fn ExpressionView() -> Element {
    let mut expr = use_signal(String::new);
    let mut eval = use_action(move || eval_expr(expr.read().clone()));
    rsx! {
        div {
            class: "flex flex-1 flex-row",
            input {
                class: "flex-1 m-1",
                placeholder: "Write here...",
                onchange: move |e: FormEvent| expr.set(e.value())
            },
            button {
                class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                onclick: move |_| eval.call(),
                "="
            },
            if let Some(result) = eval.value() {
                match result {
                    Ok(result) => rsx! {
                        p {
                            class: "m-1 p-2",
                            "{result.read().result}"
                        }
                    },
                    Err(_) => rsx! {
                        p {
                            class: "m-1 p-2",
                            { "Internal server error" }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn HistoryView() -> Element {
    let mut fetch = use_action(eval_history);
    fetch.call();
    rsx! {
        div {
            button {
                class: "rounded-xl m-1 p-2 hover:bg-slate-200",
                onclick: move |_| fetch.call(),
                "Refresh"
            },
            table {
                class: "m-1 p-2",
                tr {
                    th {
                        class: "m-1 p-2",
                        "Expression"
                    },
                    th {
                        class: "m-1 p-2",
                        "Result"
                    }
                },
                for expr in expressions(fetch) {
                    tr {
                        td {
                            class: "m-1 p-2",
                            "{expr.expression}"
                        },
                        td {
                            class: "m-1 p-2",
                            "{expr.result}"
                        }
                    }
                }
            }
        }
    }
}

fn expressions(fetch: Action<(), Vec<EvaluatedExpression>>) -> Vec<EvaluatedExpression> {
    if let Some(Ok(result)) = fetch.value() {
        result
            .read()
            .iter()
            .map(EvaluatedExpression::clone)
            .collect()
    } else {
        vec![]
    }
}

#[component]
fn CurrentUser(name: Resource<Result<String>>) -> Element {
    rsx! {
        div {
            class: "flex-1 m-3",
            match &*name.read_unchecked() {
                Some(Ok(user)) => rsx! { p { "Hello, {user}" } },
                Some(Err(err)) => rsx! { p { "Error: {err}" } },
                None => rsx! { p { "Loading..." } },
            }
        }
    }
}
