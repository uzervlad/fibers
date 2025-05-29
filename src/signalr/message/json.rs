use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{InvocationMessage, Message};

fn get_field<'a, E: serde::de::Error>(value: &'a Value, index: &'static str) -> Result<&'a Value, E> {
  value.get(index)
    .ok_or_else(|| serde::de::Error::missing_field(index))
}

impl<'de> Deserialize<'de> for Message {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>
  {
    let value = Value::deserialize(deserializer)?;

    let ty = get_field(&value, "type")?;

    let Some(ty_num) = ty.as_u64() else {
      // TODO: change to ::invalid_type / ::invalid_value
      return Err(serde::de::Error::custom("fuck"));
    };

    Ok(match ty_num {
      1 => Message::Invocation(InvocationMessage {
        invocation_id: match value.get("invocationId") {
          Some(Value::String(s)) => Some(s.clone()),
          _ => None
        },
        arguments: match get_field(&value, "arguments")? {
          Value::Array(a) => a.iter().map(|v| v.into()).collect(),
          // TODO: change to ::invalid_type
          _ => return Err(serde::de::Error::custom("fuck"))
        },
        target: get_field(&value, "target")?
          .as_str()
          // TODO: change to ::invalid_type
          .ok_or_else(|| serde::de::Error::custom("fuck"))?
          .into(),
      }),
      6 => Message::Ping,
      // TODO: change to ::invalid_value
      _ => return Err(serde::de::Error::custom("fuck")),
    })
  }
}

impl Serialize for Message {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
    let mut value = serde_json::Map::new();

    match self {
      Self::Invocation(invocation) => {
        value.insert("type".into(), Value::from(1));
        value.insert("invocationId".into(), Value::from(invocation.invocation_id.clone()));
        value.insert("target".into(), Value::from(invocation.target.clone()));
        value.insert("arguments".into(), Value::Array(
          invocation.arguments.iter()
            .map(|a| a.into())
            .collect()
        ));
      },
      Self::Completion(completion) => {
        value.insert("type".into(), Value::from(3));
        value.insert("invocationId".into(), Value::from(completion.invocation_id.clone()));
        if let Some(result) = &completion.result {
          value.insert("result".into(), result.into());
        }
        if let Some(error) = &completion.error {
          value.insert("error".into(), Value::from(error.clone()));
        }
      },
      Self::Ping => {
        value.insert("type".into(), Value::from(6));
      },
      _ => todo!()
    }

    Value::Object(value).serialize(serializer)
  }
}