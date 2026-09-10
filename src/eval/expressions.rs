use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait Expressions: Debug + Send + Sync + 'static {
    // TODO
}

pub type DynExpressions = Arc<dyn Expressions>;

#[async_trait]
impl Expressions for PgPool {
    // TODO
}

#[cfg(test)]
pub mod tests {
    // TODO
}
