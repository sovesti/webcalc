pub(crate) mod api;
#[cfg(any(feature = "server", test))]
mod evaluator;
#[cfg(feature = "server")]
pub(crate) mod expressions;
#[cfg(any(feature = "server", test))]
mod lexer;
#[cfg(any(feature = "server", test))]
mod parser;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EvaluatedExpression {
    pub expression: String,
    pub result: String,
}
