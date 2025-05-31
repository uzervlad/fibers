use axum::{body::Body, extract::{ws::WebSocket, WebSocketUpgrade}, response::Response};

pub async fn notifications_upgrade(ws: WebSocketUpgrade) -> Response<Body> {
  ws.on_upgrade(notifications_ws)
}

async fn notifications_ws(mut ws: WebSocket) {
  while let Some(Ok(msg)) = ws.recv().await {
    println!("Ws notification: {:?}", msg);
    // match msg {

    // }
  }
}
