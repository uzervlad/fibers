use axum::{body::Bytes, extract::{Multipart, Path, State}, middleware, routing::{get, post, put}, Extension, Form, Json, Router};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use sqlx::prelude::FromRow;

use crate::{auth::{self, User}, state::FiberState};

#[derive(Serialize)]
pub struct ApiUser {
  avatar_url: String,
  country_code: String,
  id: u32,
  is_active: bool,
  is_bot: bool,
  is_deleted: bool,
  is_online: bool,
  is_supporter: bool,
  last_visit: Option<String>,
  pm_friends_only: bool,
  username: String,
  cover_url: String,
  has_supported: bool,
  join_date: String,
  session_verified: bool,
  statistics_rulesets: Option<StatisticsRulesets>,
}

#[derive(Serialize)]
struct StatisticsRulesets {
  osu: Statistics,
  taiko: Statistics,
  fruits: Statistics,
  mania: Statistics,
}

#[derive(Serialize)]
struct Statistics {
  level: StatisticsLevel,
  is_ranked: bool,
  global_rank: u32,
  country_rank: u32,
  pp: f32,
  ranked_score: u32,
  accuracy: f32,
  play_count: u32,
  play_time: u32,
  total_score: u32,
  total_hits: u32,
  maximum_combo: u32,
  replays_watched_by_others: u32,
  grade_counts: StatisticsGrades
}

impl Statistics {
  fn new(stats: &DbStatistics) -> Self {
    Self {
      level: StatisticsLevel {
        current: stats.level.floor() as u16,
        progress: stats.level.fract(),
      },
      is_ranked: true,
      global_rank: 1,
      country_rank: 1,
      pp: stats.pp,
      ranked_score: 0,
      accuracy: 0.,
      play_count: stats.playcount,
      play_time: 0,
      total_score: 0,
      total_hits: 0,
      maximum_combo: 0,
      replays_watched_by_others: 0,
      grade_counts: StatisticsGrades {
        ssh: stats.ssh,
        ss: stats.ss,
        sh: stats.sh,
        s: stats.s,
        a: stats.a,
      }
    }
  }
}

#[derive(Serialize)]
struct StatisticsLevel {
  current: u16,
  progress: f32,
}

#[derive(Serialize)]
struct StatisticsGrades {
  ssh: u32,
  ss: u32,
  sh: u32,
  s: u32,
  a: u32,
}

impl ApiUser {
  pub fn new(user: &User) -> Self {
    Self {
      avatar_url: "https://f.octo.moe/files/ddd854e741d78037.png".into(), // ass pic
      country_code: "JP".into(),
      id: user.id as u32,
      is_active: true,
      is_bot: true,
      is_deleted: false,
      is_online: true,
      is_supporter: true,
      last_visit: None,
      pm_friends_only: false,
      username: user.username.clone(),

      cover_url: "https://f.octo.moe/files/0c5ea11c11244cc2.jpg".into(),
      has_supported: true,
      join_date: user.joined_at.to_rfc3339(),

      session_verified: true,
      statistics_rulesets: None,
    }
  }

  fn with_statistics(mut self, statistics: StatisticsRulesets) -> Self {
    self.statistics_rulesets = Some(statistics);

    self
  }
}

async fn me(
  Extension(user): Extension<User>,
) -> Json<ApiUser> {
  Json(ApiUser::new(&user))
}

#[derive(Clone, FromRow)]
struct DbStatistics {
  user_id: i64,
  ruleset_id: u32,
  level: f32,
  pp: f32,
  playcount: u32,
  ssh: u32,
  ss: u32,
  sh: u32,
  s: u32,
  a: u32
}

impl DbStatistics {
  fn new(user_id: i64, ruleset_id: u32) -> Self {
    Self {
      user_id,
      ruleset_id,
      level: 0.,
      pp: 0.,
      playcount: 0,
      ssh: 0,
      ss: 0,
      sh: 0,
      s: 0,
      a: 0,
    }
  }
}

async fn get_user(
  State(state): State<FiberState>,
  Path(id): Path<i64>,
) -> Json<ApiUser> {
  let user = sqlx::query_as::<_, User>(r#"
    select * from users
    where id = ?
  "#)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .unwrap();
  // let user = sqlx::query_as::<_, User>(r#"
  //   select * from users
  //   where id = ?
  // "#)
  //   .bind(id)
  //   .fetch_one(&state.pool)
  //   .await
  //   .unwrap();

   let stats = sqlx::query_as::<_, DbStatistics>(r#"
     select * from statistics
     where user_id = ?
   "#)
     .bind(id)
     .fetch_all(&state.pool)
     .await
     .unwrap();

  let stats = StatisticsRulesets {    
    osu: Statistics::new(&stats.iter().find(|s| s.ruleset_id == 0)
      .cloned()
      .unwrap_or_else(|| DbStatistics::new(user.id, 0))),
    taiko: Statistics::new(&stats.iter().find(|s| s.ruleset_id == 1)
      .cloned()
      .unwrap_or_else(|| DbStatistics::new(user.id, 1))),
    fruits: Statistics::new(&stats.iter().find(|s| s.ruleset_id == 2)
      .cloned()
      .unwrap_or_else(|| DbStatistics::new(user.id, 2))),
    mania: Statistics::new(&stats.iter().find(|s| s.ruleset_id == 3)
      .cloned()
      .unwrap_or_else(|| DbStatistics::new(user.id, 3))),
  };

  let response = ApiUser::new(&user)
    .with_statistics(stats);

  Json(response)
}


// HACK: Hacky as hell, will leave for now, maybe make it more
// generalized in the near future or think how to handle
// it bit better
fn serialize_option_as_zero<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_u32(*v),
        None => serializer.serialize_u32(0),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SoloScoresSubmitStatistic {
    #[serde(serialize_with = "serialize_option_as_zero")]
    perfect: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    great: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    ok: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    meh: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    miss: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    large_tick_miss: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    large_tick_hit: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    ignore_miss: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    ignore_hit: Option<u32>,

    #[serde(serialize_with = "serialize_option_as_zero")]
    slider_tail_hit: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SoloScoresSubmit {
    ruleset_id: u8,
    passed: bool,
    total_score: u64,
    total_score_without_mods: u64,
    accuracy: f32,
    max_combo: u32,
    rank: String,
    ranked: bool,
    statistics: SoloScoresSubmitStatistic,
    maximum_statistics: SoloScoresSubmitStatistic,

    // When submitting scores those are 
    // two parameters that are filled
    // after score was successfully submitted/processed by the server
    id: Option<i64>,
    // position: Option<i64>
}

#[derive(sqlx::FromRow)]
struct ScoreDb {
    online_id: i64,
    data_json: Vec<u8>,
}

async fn put_solo_scores(
    State(state): State<FiberState>,
    Path((beatmap_id, token)): Path<(i64, i64)>,
    body: Bytes,
) -> Json<SoloScoresSubmit> {
    // Check if this score is indeed in flight
    let lock = state.in_flight_scores.read().await;

    if !lock.contains(&token) {
        // TODO: A case when we got score submission without
        // any token retrival
        unimplemented!();
    }
    drop(lock);

    let mut lock = state.in_flight_scores.write().await;
    lock.remove(&token);
    drop(lock);

    // Handle storing and returning back to the client

    // Currently that's a some sort of verifier before inserting to the
    // database
    // TODO: Think about make it a bit efficient?
    let _payload: SoloScoresSubmit = serde_json::from_slice(&body).unwrap();

    let payload_str = str::from_utf8(&body).unwrap();
    
    // Insert to retrieve online_id
    let record = sqlx::query!(
        "INSERT INTO scores (data_json) VALUES (?1) RETURNING *",
        payload_str
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let mut payload_from_db: SoloScoresSubmit = serde_json::from_slice(&record.data_json).unwrap();
    
    payload_from_db.id = Some(record.online_id);

    Json(payload_from_db)
}

#[derive(Debug, TryFromMultipart)]
struct SoloScoresTokenSubmit {
    version_hash: String,
    beatmap_hash: String,
    ruleset_id: i32
}

/// Assign a token for the score submission
///
/// Expected response -> `{ id: long }`
async fn solo_score_token(
  Extension(user): Extension<User>,
  State(state): State<FiberState>,
  Path(beatmap_id): Path<i64>,
  multipart: TypedMultipart<SoloScoresTokenSubmit>,
) -> Json<serde_json::Value> {
    println!("Score token retrival for {}", user.id);

    let token: i64 = rand::random();
    
    {
        let mut lock = state.in_flight_scores.write().await;
        lock.insert(token);
    }

    Json(json!{{
        "id": token
    }})
}

pub fn router(state: FiberState) -> Router<FiberState> {
  Router::new()
    .route("/api/v2/me/", get(me))
    .route("/api/v2/users/{id}/", get(get_user))
    .route("/api/v2/beatmaps/{beatmap_id}/solo/scores", post(solo_score_token))
    .route("/api/v2/beatmaps/{beatmap_id}/solo/scores/{token}", put(put_solo_scores))
    .layer(middleware::from_fn_with_state(state, auth::middleware))
}
