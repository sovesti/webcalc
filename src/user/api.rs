#[cfg(feature = "server")]
use axum::{Extension, http::Method};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::{
    auth::{AxumSession, check_permission},
    permission::Permission,
    users::DynUsers,
};

#[post("/user/signin", auth: AxumSession, Extension(users): Extension<DynUsers>)]
pub async fn sign_in(username: String, password: String) -> Result<()> {
    auth.login_user(users.sign_in(username, password).await?.id());
    Ok(())
}

#[post("/user/signout", auth: AxumSession)]
pub async fn sign_out() -> Result<()> {
    auth.logout_user();
    Ok(())
}

#[post("/user/signup", Extension(users): Extension<DynUsers>)]
pub async fn sign_up(username: String, password: String) -> Result<()> {
    users.sign_up(username, password).await?;
    Ok(())
}

#[get("/user/name", auth: AxumSession, Extension(users): Extension<DynUsers>)]
pub async fn user_name() -> Result<String> {
    check_permission(&users, Method::GET, Permission::default(), &auth).await?;
    Ok(users.find(auth.id).await?.name().to_owned())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use async_trait::async_trait;       
    use axum_test::{TestServer, TestResponse};
    use serde_json::{json, to_value};
    use crate::backend::tests::Response;
    
    pub trait UsernameResponse : Response {
        fn assert_exact(&self, value: &str);
    }

    #[async_trait]
    impl UsernameResponse for TestResponse {
        fn assert_exact(&self, value: &str) {
            self
                .assert_status_ok()
                .assert_json(&json!({"username": value}));
        }
    }

    #[async_trait]
    pub trait Api {
        async fn sign_up(&self, username: &str, password: &str) -> impl Response;

        async fn sign_in(&self, username: &str, password: &str) -> impl Response;

        async fn sign_out(&self) -> impl Response;

        async fn user_name(&self) -> impl UsernameResponse;
    }

    #[async_trait]
    impl Api for TestServer {
        async fn sign_in(&self, username: &str, password: &str) -> impl Response {
            self
                .post("/user/signin")
                .json(&json! ({
                    "username": username,
                    "password": password
                }))
                .await
        }
        
        async fn sign_up(&self, username: &str, password: &str) -> impl Response {
            self
                .post("/user/signup")
                .json(&json! ({
                    "username": username,
                    "password": password
                }))
                .await
        }

        async fn sign_out(&self) -> impl Response {
            self.post("/user/signout").await
        }

        async fn user_name(&self) -> impl UsernameResponse {
            self.get("/user/name").await
        }
    }
}
