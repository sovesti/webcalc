pub(crate) mod api;
#[cfg(any(feature = "server", test))]
mod evaluator;
#[cfg(feature = "server")]
pub(crate) mod expressions;
#[cfg(any(feature = "server", test))]
mod lexer;
#[cfg(any(feature = "server", test))]
mod parser;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EvaluatedExpression {
    pub expression: String,
    pub result: String,
}

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

impl EvaluatedExpression {
    #[cfg(test)]
    pub fn new(expression: &str, result: &str) -> Self {
        Self {
            expression: expression.to_owned(),
            result: result.to_owned(),
        }
    }
}
