use std::{collections::HashSet, sync::Arc};

use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::sync::RwLock;

pub type FiberState = Arc<FiberStateInner>;

pub struct FiberStateInner {
  pub pool: Pool<Sqlite>,

  /// Scores that are in-flight
  //  TODO: those should have TTL? Since there are
  //  scores which tokens are retrived but never
  //  submited (e.g. when score = 0)
  pub in_flight_scores: RwLock<HashSet<i64>>,
}

impl FiberStateInner {
  pub async fn new() -> Result<Self> {
    Ok(Self {
        pool: SqlitePool::connect("sqlite:fibers.db").await?,
        in_flight_scores: Default::default(),
    })
  }
}
