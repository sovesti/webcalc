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
            class: "flex flex-col gap-4",
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
            class: "flex flex-1 flex-row gap-4",
            input {
                class: "flex-1 m-1 self-center rounded-xl border border-slate-400 p-2",
                placeholder: "Write here...",
                onchange: move |e: FormEvent| expr.set(e.value())
            },
            button {
                class: "rounded-xl m-1 px-4 py-2 bg-blue-600 text-white hover:bg-blue-700 active:bg-blue-800 cursor-pointer transition-colors self-center",
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
                    Err(err) => match err.0.downcast_ref::<ServerFnError>() {
                        Some(ServerFnError::ServerError { code: 400, message, .. }) => rsx! {
                            p {
                                class: "m-1 p-2 rounded-xl bg-red-500 text-white",
                                "{message}"
                            }
                        },
                        _ => rsx! {
                            p {
                                class: "m-1 p-2",
                                { "Internal server error" }
                            }
                        },
                    },
                }
            }
        }
    }
}

#[component]
fn HistoryView() -> Element {
    let mut fetch = use_action(eval_history);
    rsx! {
        div {
            button {
                class: "rounded-xl m-1 px-4 py-2 bg-blue-600 text-white hover:bg-blue-700 active:bg-blue-800 cursor-pointer transition-colors",
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
