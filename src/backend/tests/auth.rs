use super::{test_server, Response};
use crate::user::api::tests::{Api, UsernameResponse};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn sign_up() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
}

#[tokio::test]
#[serial]
async fn sign_up_out_in_out() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_out().await.assert_ok();
    ts.sign_in("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_out().await.assert_ok();
}

#[tokio::test]
#[serial]
async fn username() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.user_name().await.assert_exact("Alice");
}

#[tokio::test]
#[serial]
async fn username_unauth() {
    let ts = test_server().await;
    ts.user_name().await.assert_unauthorized();
}

#[tokio::test]
#[serial]
async fn sign_in_when_signed_in() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_in("Bob", "HESOYAM").await.assert_forbidden();
}

#[tokio::test]
#[serial]
async fn sign_out_when_not_signed_in() {
    let ts = test_server().await;
    ts.sign_out().await.assert_forbidden();
}

#[tokio::test]
#[serial]
async fn sign_up_same_username() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_out().await.assert_ok();
    ts.sign_up("Alice", "HESOYAM").await.assert_forbidden();
}

#[tokio::test]
#[serial]
async fn sign_up_same_password() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_out().await.assert_ok();
    ts.sign_up("Bob", "AEZAKMI").await.assert_ok();
}

#[tokio::test]
#[serial]
async fn sign_in_wrong_username() {
    let ts = test_server().await;
    ts.sign_up("Alice", "AEZAKMI").await.assert_ok();
    ts.sign_out().await.assert_ok();
    ts.sign_in("Alice", "HESOYAM").await.assert_unauthorized();
}
