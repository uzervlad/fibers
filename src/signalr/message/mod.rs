use super::value::SignalRValue;

pub mod json;
pub mod msgpack;

#[derive(Debug)]
pub enum Message {
  Close,
  Invocation(InvocationMessage),
  StreamInvocation,
  StreamItem,
  Completion(CompletionMessage),
  CancelInvocation,
  Ping,
  Ack,
  Sequence,
}

#[derive(Debug)]
pub struct InvocationMessage {
  pub invocation_id: Option<String>,
  pub target: String,
  pub arguments: Vec<SignalRValue>,
}

#[derive(Debug)]
pub struct CompletionMessage {
  pub invocation_id: String,
  pub result: Option<SignalRValue>,
  pub error: Option<String>,
}