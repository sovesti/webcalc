mod auth;
mod eval;

use super::test_router;

use axum_test::{TestServer, TestResponse};

pub async fn test_server() -> TestServer {
    TestServer::new(test_router().await.unwrap())
}

pub trait Response {
    fn assert_ok(&self);

    fn assert_unauthorized(&self);

    fn assert_forbidden(&self);
}

impl Response for TestResponse {
    fn assert_ok(&self) {
        self.assert_status_ok();
    }

    fn assert_unauthorized(&self) {
        self.assert_status_not_ok();
    }

    fn assert_forbidden(&self) {
        self.assert_status_not_ok();
    }
}

