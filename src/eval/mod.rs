pub(crate) mod api;
#[cfg(feature = "server")]
pub(crate) mod expressions;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EvaluatedExpression {
    expression: String,
    result: String,
}
