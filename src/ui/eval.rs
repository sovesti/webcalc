use dioxus::prelude::*;

use crate::{
    eval::{
        EvaluatedExpression,
        api::{EvalResponse, eval_expr, eval_history},
    },
    user::api::user_name,
};

type FetchExpressions = Action<(), Vec<EvaluatedExpression>>;

#[component]
pub fn EvalView() -> Element {
    let name = use_resource(user_name);
    let mut fetch = use_action(eval_history);
    spawn(async move {
        fetch.call();
    });
    rsx! {
        div {
            class: "flex flex-col gap-4",
            CurrentUser { name },
            ExpressionView { fetch },
            HistoryView { fetch }
        }
    }
}

#[component]
fn ExpressionView(fetch: FetchExpressions) -> Element {
    let mut expr = use_signal(String::new);
    let mut eval = use_action(move || eval_and_refresh(expr, fetch));
    rsx! {
        div {
            class: "flex flex-1 flex-row gap-3",
            input {
                class: "flex-8 m-1 self-center rounded-xl border border-slate-400 p-2",
                placeholder: "Write here...",
                onchange: move |e: FormEvent| expr.set(e.value())
            },
            button {
                class: "flex-1 rounded-xl m-1 px-4 py-2 bg-sky-600 text-white hover:bg-sky-700 active:bg-sky-800 cursor-pointer transition-colors self-center",
                onclick: move |_| eval.call(),
                "="
            },
            div {
                class: "flex-4 m-1 p-2",
                if let Some(result) = eval.value() {
                    match result {
                        Ok(result) => rsx! {
                            p {
                                "{result.read().result}"
                            }
                        },
                        Err(err) => match err.0.downcast_ref::<ServerFnError>() {
                            Some(ServerFnError::ServerError { code: 400, message, .. }) => rsx! {
                                p {
                                    class: "rounded-xl text-red-500",
                                    "{message}"
                                }
                            },
                            _ => rsx! {
                                p {
                                    "Internal server error"
                                }
                            },
                        },
                    }
                }
            }
        }
    }
}

async fn eval_and_refresh(
    expr: Signal<String>,
    mut fetch: FetchExpressions,
) -> Result<EvalResponse> {
    let resp = eval_expr(expr.read().to_string()).await?;
    fetch.call().await;
    Ok(resp)
}

#[component]
fn HistoryView(fetch: FetchExpressions) -> Element {
    rsx! {
        div {
            class: "w-9/13",
            button {
                class: "rounded-xl m-1 px-4 py-2 bg-sky-600 text-white hover:bg-sky-700 active:bg-sky-800 cursor-pointer transition-colors",
                onclick: move |_| fetch.call(),
                "Refresh"
            },
            table {
                class: "p-2 m-1 table-fixed overflow-scroll w-full",
                thead {
                    class: "w-full",
                    tr {
                        th {
                            class: "m-1 p-2 w-8/9 text-center",
                            "Expression"
                        },
                        th {
                            class: "m-1 p-2 w-1/9 text-center",
                            "Result"
                        }
                    }
                },
                tbody {
                    Expressions { fetch }
                }
            }
        }
    }
}

#[component]
fn Expressions(fetch: FetchExpressions) -> Element {
    rsx! {
        for expr in expressions(fetch) {
            tr {
                td {
                    class: "m-1 p-2 text-center",
                    "{expr.expression}"
                },
                td {
                    class: "m-1 p-2 text-center",
                    "{expr.result}"
                }
            }
        }
    }
}

fn expressions(fetch: FetchExpressions) -> Vec<EvaluatedExpression> {
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
