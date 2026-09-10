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

#[post("/user/signup")]
pub async fn sign_up() -> Result<()> {
    todo!()
}

#[get("/user/name", auth: AxumSession, Extension(users): Extension<DynUsers>)]
pub async fn user_name() -> Result<String> {
    check_permission(&users, Method::GET, Permission::default(), &auth).await?;
    Ok(users.find(auth.id).await?.name().to_owned())
}
