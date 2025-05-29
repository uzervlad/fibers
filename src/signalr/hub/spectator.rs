use axum::extract::ws::{self, WebSocket};

use crate::signalr::{hub::send_msgpack, message::{msgpack::deserialize_message, CompletionMessage, Message}};

use super::{initiate, SignalRProtocol};

pub async fn handle_spectator_hub(mut socket: WebSocket) {
  let protocol = match initiate(&mut socket).await {
    Ok(protocol) => protocol,
    Err(e) => return eprintln!("{:?}", e),
  };

  if protocol != SignalRProtocol::Msgpack {
    return;
  }

  println!("[spectator] New connection");

  while let Some(Ok(msg)) = socket.recv().await {
    match msg {
      ws::Message::Binary(data) => {
        // let Ok(msg) = deserialize_message(&data) else {
        //   continue
        // };
        let msg = match deserialize_message(&data) {
          Ok(msg) => msg,
          Err(e) => {
            eprintln!("{}", e.backtrace());
            continue
          }
        };

        match msg {
          Message::Invocation(invocation) => {
            println!("[spectator] Invoked {} with {:?}", invocation.target, invocation.arguments);

            // ...

            if let Some(id) = invocation.invocation_id {
              let completion = Message::Completion(CompletionMessage {
                invocation_id: id,
                result: None,
                error: None,
              });

              send_msgpack(&mut socket, completion).await;
            }
          },
          Message::Ping => {
            println!("[spectator] Ping");

            let ping = Message::Ping;

            send_msgpack(&mut socket, ping).await;
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