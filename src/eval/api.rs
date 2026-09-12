#[cfg(feature = "server")]
use crate::user::{auth::check_permission, permission::Permission, users::DynUsers};
#[cfg(feature = "server")]
use axum::{
    Extension,
    http::{Method, StatusCode},
};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::eval::EvaluatedExpression;
#[cfg(feature = "server")]
use crate::{
    eval::{evaluator::evaluate_expression, expressions::DynExpressions},
    user::auth::AxumSession,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct EvalResponse {
    pub result: String,
}

#[post("/eval", auth: AxumSession, Extension(users): Extension<DynUsers>, Extension(expressions): Extension<DynExpressions>)]
pub async fn eval_expr(expression: String) -> Result<EvalResponse> {
    check_permission(&users, Method::POST, Permission::default(), &auth)
        .await
        .or_forbidden("Permission denied")?;
    let result = evaluate_expression(&expression)
        .map_err(|error| HttpError::new(StatusCode::BAD_REQUEST, error.to_string()))?
        .to_string();
    record_expression(auth.id, expression, result.clone(), expressions).await?;
    Ok(EvalResponse { result })
}

#[cfg(feature = "server")]
async fn record_expression(
    id: i32,
    expression: String,
    result: String,
    expressions: DynExpressions,
) -> Result<()> {
    expressions
        .add_expression(id, EvaluatedExpression { expression, result })
        .await?;
    Ok(())
}

#[get("/eval", auth: AxumSession, expressions: Extension<DynExpressions>)]
pub async fn eval_history() -> Result<Vec<EvaluatedExpression>> {
    Ok(expressions.history(auth.id).await?)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use async_trait::async_trait;       
    use axum_test::{TestServer, TestResponse};
    use serde_json::{json, to_value};
    use crate::backend::tests::Response;

    pub trait EvalHistoryResponse : Response {
        fn assert_exact(&self, value: Vec<EvaluatedExpression>);
    }

    impl EvalHistoryResponse for TestResponse {
        fn assert_exact(&self, value: Vec<EvaluatedExpression>) {
            self
                .assert_status_ok()
                .assert_json(&to_value(value).unwrap());
        }
    }

    #[async_trait]
    pub trait Api {
        async fn eval_expr(&self, expression: &str) -> TestResponse;

        async fn eval_history(&self) -> impl EvalHistoryResponse;
    }

    #[async_trait]
    impl Api for TestServer {
        async fn eval_expr(&self, expression: &str) -> TestResponse {
            self.post("/eval").json(&json!({"expression": expression})).await
        }

        async fn eval_history(&self) -> impl EvalHistoryResponse {
            self.get("/eval").await
        }
    }
}
