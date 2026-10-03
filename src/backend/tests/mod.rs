mod auth;
mod eval;

use super::test_router;

use async_once::AsyncOnce;
use axum_test::{TestResponse, TestServer};
use lazy_static::lazy_static;

use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref TEST_SERVER: AsyncOnce<Mutex<TestServer>> =
        AsyncOnce::new(async { Mutex::new(make_test_server().await) });
}

async fn make_test_server() -> TestServer {
    println!("make test server");
    TestServer::new(test_router().await.unwrap())
}

pub async fn test_server<'a>() -> MutexGuard<'a, TestServer> {
    // silence posion errors, because integration test threads will usually panic with locked
    // test server mutex
    TEST_SERVER
        .get()
        .await
        .lock()
        .map_or_else(|e| e.into_inner(), |ts| ts)
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
