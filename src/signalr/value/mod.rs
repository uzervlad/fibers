use std::collections::HashMap;

pub mod json;
pub mod msgpack;

#[derive(Debug)]
pub enum SignalRValue {
  Integer(i64),
  Float(f64),
  String(String),
  Boolean(bool),
  Array(Vec<SignalRValue>),
  Object(HashMap<String, SignalRValue>),
  Null,
}