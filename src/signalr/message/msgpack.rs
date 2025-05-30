use std::{error::Error, fmt::Display};

use anyhow::{anyhow, Result};
use rmpv::Value;

use super::{InvocationMessage, Message};

#[derive(Debug)]
pub enum MsgpackParseError {
  InvalidType,
}

impl Display for MsgpackParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for MsgpackParseError {}

// TODO: rename this
pub fn parse_message_with_length(buf: &[u8]) -> &[u8] {
  let mut length = 0;
  
  for i in 0..5 {
    let byte = buf[i];

    length |= (byte as usize & 0x7F) << (i * 7);

    if (byte & 0x80) == 0 {
      return &buf[i + 1..][..length];
    }
  }

  unreachable!();
}

pub fn deserialize_message(buf: &[u8]) -> Result<Message> {
  let mut buf = parse_message_with_length(buf);

  let value = rmpv::decode::read_value(&mut buf)?;

  let Value::Array(mut data) = value else {
    unreachable!()
  };

  let Value::Integer(ty) = data.remove(0) else {
    return Err(anyhow!("fuck"))
  };

  //println!("Deser message: {:?} - {:?}", ty, data);

  Ok(match ty.as_u64().unwrap() {
    1 => {
      let _headers = data.remove(0);

      Message::Invocation(InvocationMessage {
        invocation_id: match data.remove(0) {
          Value::Nil => None,
          Value::String(s) => Some(s.to_string()),
          _ => return Err(MsgpackParseError::InvalidType.into())
        },
        target: data.remove(0)
          .as_str()
          .ok_or_else(|| MsgpackParseError::InvalidType)?
          .into(),
        arguments: data.remove(0)
          .as_array()
          .ok_or_else(|| MsgpackParseError::InvalidType)?
          .iter()
          .map(|v| v.into())
          .collect()
      })
    },
    6 => {
      Message::Ping
    }
    _ => return Err(anyhow!("fuck"))
  })
}

fn serialize_varint(mut n: usize) -> Vec<u8> {
  let mut v = vec![];

  while n > 0 {
    let mut byte = (n & 0x7F) as u8;
    n >>= 7;
    if n > 0 {
      byte |= 0x80;
    }

    v.push(byte);
  }

  v
}

pub fn serialize_message(message: &Message) -> Result<Vec<u8>> {
  let mut buf = vec![];

  let value = match message {
    Message::Completion(completion) => {
      let result_kind = match (&completion.result, &completion.error) {
        (_, Some(_)) => 1,
        (None, _) => 2,
        (Some(_), _) => 3,
      };

      Value::Array(vec![
        Value::from(3),
        Value::Map(vec![]),
        Value::String(completion.invocation_id.clone().into()),
        Value::from(result_kind),
        match result_kind {
          1 => completion.error.clone().unwrap().into(),
          2 => Value::Nil,
          3 => completion.result.as_ref().unwrap().into(),
          _ => unreachable!()
        }
      ])
    },
    Message::Ping => {
      Value::Array(vec![
        Value::from(6)
      ])
    }
    _ => todo!()
  };

  rmpv::encode::write_value(&mut buf, &value)?;

  let mut final_buf = serialize_varint(buf.len());

  final_buf.append(&mut buf);

  Ok(final_buf)
}
