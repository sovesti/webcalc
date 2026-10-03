pub(crate) mod config;

#[cfg(test)]
pub mod tests;

use std::sync::Arc;

use axum::Extension;
use dioxus::prelude::dioxus_fullstack::routing::Router;
#[cfg(test)]
use dioxus::server::{DioxusRouterExt, FullstackState, axum::http::StatusCode};

use crate::{
    backend::config::Config,
    db,
    eval::expressions::DynExpressions,
    ui::app,
    user::{auth, users::DynUsers},
};

pub async fn router() -> anyhow::Result<Router> {
    let config = Config::load()?;
    let db = config.postgres().connect().await?;
    db::initialize_postgres(&db).await?;
    let arc = Arc::new(db.clone());
    Ok(dioxus::server::router(app)
        .layer(auth::auth_layer(arc.clone()))
        .layer(auth::auth_session_layer(db.clone()).await?)
        .layer(Extension(arc.clone() as DynUsers))
        .layer(Extension(arc.clone() as DynExpressions)))
}

#[cfg(test)]
pub async fn test_router() -> anyhow::Result<Router> {
    let config = Config::load()?;
    println!("start connetcing to db with config {config:?}");
    let db = config.postgres().connect().await?;
    println!("connected to db");
    // for tests create empty database
    db::drop_tables(&db).await?;
    db::initialize_postgres(&db).await?;
    let arc = Arc::new(db.clone());
    let router = dioxus::server::axum::Router::new()
        .register_server_functions()
        .layer(auth::auth_layer(arc.clone()))
        .layer(auth::auth_session_layer(db.clone()).await?)
        .layer(Extension(arc.clone() as DynUsers))
        .layer(Extension(arc.clone() as DynExpressions))
        .fallback(async || (StatusCode::NOT_FOUND, "not found"))
        .with_state(FullstackState::headless());
    Ok(router)
}
