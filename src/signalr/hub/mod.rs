use std::{error::Error, fmt::Display};

use anyhow::Result;
use axum::extract::ws::{self, WebSocket};
use serde::Deserialize;

use super::message::{msgpack::serialize_message, Message};

pub mod metadata;
pub mod multiplayer;
pub mod spectator;

#[derive(Debug, PartialEq)]
pub enum SignalRProtocol {
  Msgpack,
  Json,
}

#[derive(Deserialize)]
pub struct SignalRHandshake {
  protocol: String,
  #[allow(unused)]
  version: u8,
}

#[derive(Debug)]
pub enum SignalRHandshakeError {
  UnknownSocketError,
  InvalidProtocol,
}

impl Display for SignalRHandshakeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for SignalRHandshakeError {}

pub async fn initiate(socket: &mut WebSocket) -> Result<SignalRProtocol> {
  let Some(msg) = socket.recv().await else {
    return Err(SignalRHandshakeError::UnknownSocketError.into());
  };
  let msg = msg?;

  let json = msg.into_data();

  // trimming 0x1e record separator at the end
  let handshake = serde_json::from_str::<SignalRHandshake>(str::from_utf8(&json[..json.len() - 1])?)?;

  socket.send(ws::Message::binary(b"{}\x1E".as_slice())).await?;

  match handshake.protocol.as_str() {
    "messagepack" => Ok(SignalRProtocol::Msgpack),
    "json" => Ok(SignalRProtocol::Json),
    _ => Err(SignalRHandshakeError::InvalidProtocol.into())
  }
}

pub async fn send_json(ws: &mut WebSocket, message: Message) {
  let Ok(json) = serde_json::to_string(&message) else {
    return
  };

  let mut bytes = json.as_bytes().to_vec();
  bytes.push(0x1E);

  let _ = ws.send(ws::Message::Binary(bytes.into())).await;
}

pub async fn send_msgpack(ws: &mut WebSocket, message: Message) {
  let Ok(bytes) = serialize_message(&message) else {
    return
  };

  let _ = ws.send(ws::Message::Binary(bytes.into())).await;
}