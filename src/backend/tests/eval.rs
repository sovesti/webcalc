use super::{test_server, Response};
use crate::{
    eval::{EvaluatedExpression, api::tests::{Api as EvalApi, EvalHistoryResponse}}, user::{api::tests::Api as UserApi, users::test},
};
use serial_test::serial;


#[tokio::test]
#[serial]
async fn eval_and_history() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.eval_expr("1 - 2 - 3").await.assert_ok();
    ts.eval_history().await.assert_exact(vec![
        EvaluatedExpression::new("1 - 2 - 3", "-4"),
    ]);
}

#[tokio::test]
#[serial]
async fn eval_history_empty() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.eval_history().await.assert_exact(vec![]);
}

#[tokio::test]
#[serial]
async fn eval_history_multiple() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.eval_expr("(1 - 2 - 3) * (4 - 5) - 6 * 6 + 8").await.assert_ok();
    ts.eval_expr("1 - 2 * 3 + (4 * 5 - 6) + (7 * -8)").await.assert_ok();
    ts.eval_expr("-1 * -2 + -(3 * 4 - (5 * -6 + 7 + 8))").await.assert_ok();
    ts.eval_expr("1              +2*-    3-     (   4 +5)/    6").await.assert_ok();
    ts.eval_history().await.assert_exact(vec![
        EvaluatedExpression::new("(1 - 2 - 3) * (4 - 5) - 6 * 6 + 8", "-24"),
        EvaluatedExpression::new("1 - 2 * 3 + (4 * 5 - 6) + (7 * -8)", "-47"),
        EvaluatedExpression::new("-1 * -2 + -(3 * 4 - (5 * -6 + 7 + 8))", "-25"),
        EvaluatedExpression::new("1              +2*-    3-     (   4 +5)/    6", "-4"),
    ]);
}

#[tokio::test]
#[serial]
async fn eval_expr_unauth() {
    let ts = test_server().await;
    ts.eval_expr("1 + 2 * 3").await.assert_unauthorized();
}

#[tokio::test]
#[serial]
async fn eval_history_unauth() {
    let ts = test_server().await;
    ts.eval_history().await.assert_unauthorized();
}

#[tokio::test]
#[serial]
async fn eval_history_different_users() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.eval_expr("1 - 2 + 3").await.assert_ok();
    ts.sign_out().await.assert_ok();
    ts.sign_up("Bob", "HESOYAM").await.assert_ok();
    ts.eval_expr("4 - 5 + 6").await.assert_ok();
    ts.eval_history().await.assert_exact(vec![
        EvaluatedExpression::new("4 - 5 + 6", "5"),
    ]);
    ts.sign_out().await.assert_ok();
    ts.sign_in("Alice", "AEZAKMI").await.assert_ok();
    ts.eval_history().await.assert_exact(vec![
        EvaluatedExpression::new("1 - 2 + 3", "2"),
    ]);
}