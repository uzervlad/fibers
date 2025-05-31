use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde::Serialize;

use crate::{auth::User, state::FiberState};

#[derive(TryFromMultipart)]
struct TokenMultipart {
  username: String,
  #[allow(unused)]
  password: String,
}

#[derive(Serialize)]
struct TokenResponse {
  access_token: String,
  refresh_token: String,
  #[serde(rename = "type")]
  ty: String,
  expires_in: u32,
}

async fn token(
  State(state): State<FiberState>,
  body: TypedMultipart<TokenMultipart>,
) -> Result<Json<TokenResponse>, StatusCode> {
  match sqlx::query_as::<_, User>(r#"
    select * from users
    where username = ?
  "#)
    .bind(&body.username)
    .fetch_one(&state.pool)
    .await
  {
    Ok(user) => Ok(Json(TokenResponse {
      access_token: user.id.to_string(),
      refresh_token: user.id.to_string(),
      ty: "Bearer".into(),
      expires_in: u32::MAX,
    })),
    Err(e) => {
      println!("select users: {:?}", e);
      match sqlx::query_scalar::<_, i64>(r#"
        insert into users (username, joined_at) values
        (?, datetime())
        returning id
      "#)
        .bind(&body.username)
        .fetch_one(&state.pool)
        .await
      {
        Ok(id) => Ok(Json(TokenResponse {
          access_token: id.to_string(),
          refresh_token: id.to_string(),
          ty: "Bearer".into(),
          expires_in: u32::MAX,
        })),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
      }
    }
  }
}

pub fn router() -> Router<FiberState> {
  Router::new()
    .route("/token", post(token))
}
