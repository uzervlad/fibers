use axum::{extract::{Request, State}, http::{header, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Extension};
use sqlx::prelude::FromRow;

use crate::state::FiberState;

#[derive(Clone, FromRow)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub joined_at: chrono::DateTime<chrono::Utc>,
}

pub type UserExtension = Extension<User>;

pub async fn middleware(
  State(state): State<FiberState>,
  mut request: Request,
  next: Next
) -> Response {
  let Some(auth) = request.headers().get(header::AUTHORIZATION) else {
    println!("no auth header");
    return StatusCode::UNAUTHORIZED.into_response()
  };

  let Ok(token) = auth.to_str() else {
    println!("token to_str failed");
    return StatusCode::UNAUTHORIZED.into_response()
  };

  println!("{}", token);

  let Some((_, id)) = token.split_once(' ') else {
    println!("token split failed");
    return StatusCode::UNAUTHORIZED.into_response()
  };

  let Ok(id) = id.parse::<i64>() else {
    println!("token parse failed");
    return StatusCode::UNAUTHORIZED.into_response()
  };

  let Ok(user) = sqlx::query_as::<_, User>(r#"
    select * from users
    where id = ?
  "#)
    .bind(id)
    .fetch_one(&state.pool)
    .await else
  {
    return StatusCode::UNAUTHORIZED.into_response()
  };

  request.extensions_mut().insert(user);

  next.run(request).await
}