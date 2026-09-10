use std::fmt::Debug;

pub(crate) mod api;
#[cfg(feature = "server")]
pub(crate) mod auth;
#[cfg(feature = "server")]
mod password;
#[cfg(feature = "server")]
pub(crate) mod permission;
#[cfg(feature = "server")]
pub(crate) mod users;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct User {
    id: i32,
    name: String,
    password: String,
}

#[cfg(feature = "server")]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
