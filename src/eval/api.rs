#[cfg(feature = "server")]
use axum::Extension;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::eval::EvaluatedExpression;
#[cfg(feature = "server")]
use crate::{eval::expressions::DynExpressions, user::auth::AxumSession};

#[derive(Deserialize, Serialize, Debug)]
pub struct EvalResponse {
    result: String,
}

#[post("/eval", auth: AxumSession, expressions: Extension<DynExpressions>)]
pub async fn eval_expr(expression: String) -> Result<()> {
    todo!()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryResponse {
    entries: Vec<EvaluatedExpression>,
}

#[get("/eval", auth: AxumSession, expressions: Extension<DynExpressions>)]
pub async fn eval_history() -> Result<Vec<EvaluatedExpression>> {
    Ok(expressions.history(auth.id).await?)
}
