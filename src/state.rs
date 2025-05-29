use std::sync::Arc;

use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};

pub type FiberState = Arc<FiberStateInner>;

pub struct FiberStateInner {
  pub pool: Pool<Sqlite>,
}

impl FiberStateInner {
  pub async fn new() -> Result<Self> {
    Ok(Self {
      pool: SqlitePool::connect("sqlite:fibers.db").await?
    })
  }
}