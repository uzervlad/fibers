use axum::extract::ws::{self, WebSocket};

use crate::signalr::{hub::{send_json, SignalRProtocol}, message::{CompletionMessage, Message}};

use super::initiate;

pub async fn handle_metadata_hub(mut socket: WebSocket) {
  let protocol = match initiate(&mut socket).await {
    Ok(protocol) => protocol,
    Err(e) => return eprintln!("{:?}", e),
  };

  if protocol != SignalRProtocol::Json {
    return;
  }

  println!("[metadata] New connection");

  while let Some(Ok(msg)) = socket.recv().await {
    match msg {
      ws::Message::Text(data) => {
        let Ok(msg) = serde_json::from_str::<Message>(&data) else {
          continue
        };

        println!("{:?}", msg);

        match msg {
          Message::Invocation(invocation) => {
            println!("[metadata] Invoked {}", invocation.target);

            // ...

            if let Some(id) = invocation.invocation_id {
              let completion = Message::Completion(CompletionMessage {
                invocation_id: id,
                result: None,
                error: None,
              });

              send_json(&mut socket, completion).await;
            }
          },
          Message::Ping => {
            println!("[metadata] Ping");

            let ping = Message::Ping;

            send_json(&mut socket, ping).await;
          },
          _ => {},
        }
      },
      ws::Message::Close(_frame) => {
        // ok :(
      },
      _ => {
        // what.
      }
    }
  }
}