use axum::{body::Body, extract::{Path, WebSocketUpgrade}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde::Serialize;

use crate::{signalr::hub::{metadata::handle_metadata_hub, multiplayer::handle_multiplayer_hub, spectator::handle_spectator_hub}, state::FiberState};

async fn signalr_hub(
  Path(hub): Path<String>,
  ws: WebSocketUpgrade
) -> Response<Body> {
  match hub.as_str() {
    "metadata" => ws.on_upgrade(handle_metadata_hub),
    "multiplayer" => ws.on_upgrade(handle_multiplayer_hub),
    "spectator" => ws.on_upgrade(handle_spectator_hub),
    _ => (StatusCode::NOT_FOUND).into_response(),
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignalRTransport {
  transport: String,
  transfer_formats: Vec<String>,
}

impl Default for SignalRTransport {
  fn default() -> Self {
    Self {
      transport: "WebSockets".into(),
      transfer_formats: vec![
        "Text".into(),
        "Binary".into(),
      ],
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignalRNegotiate {
  connection_token: String,
  connection_id: String,
  negotiate_version: u8,
  available_transports: Vec<SignalRTransport>,
}

impl Default for SignalRNegotiate {
  fn default() -> Self {
    Self {
      connection_token: "connectionToken".into(),
      connection_id: "connectionId".into(),
      negotiate_version: 1,
      available_transports: vec![
        SignalRTransport::default(),
      ],
    }
  }
}

async fn signalr_negotiate() -> Json<SignalRNegotiate> {
  Json(SignalRNegotiate::default())
}


pub fn router() -> Router<FiberState> {
  Router::new()
    .route("/{hub}", get(signalr_hub))
    .route("/{hub}/negotiate", post(signalr_negotiate))
}