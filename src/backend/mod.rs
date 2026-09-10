pub(crate) mod config;

use std::sync::Arc;

use axum::Extension;
use dioxus::prelude::dioxus_fullstack::routing::Router;

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
