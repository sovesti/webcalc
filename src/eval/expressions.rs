use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, query_as};

use super::EvaluatedExpression;

#[async_trait]
pub trait Expressions: Debug + Send + Sync + 'static {
    async fn history(&self, account_id: i32) -> Result<Vec<EvaluatedExpression>>;
}

pub type DynExpressions = Arc<dyn Expressions>;

#[async_trait]
impl Expressions for PgPool {
    async fn history(&self, account_id: i32) -> Result<Vec<EvaluatedExpression>> {
        Ok(query_as::<_, EvaluatedExpression>(
            r#"
            SELECT expression, result
            FROM evaluated_expression
            WHERE account = $1
            ORDER BY id DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(self)
        .await?)
    }
}

#[cfg(test)]
pub mod tests {
    // TODO
}