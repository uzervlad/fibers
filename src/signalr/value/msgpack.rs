use super::SignalRValue;

impl Into<rmpv::Value> for &SignalRValue {
  fn into(self) -> rmpv::Value {
    match self {
      SignalRValue::Integer(n) => rmpv::Value::Integer(rmpv::Integer::from(*n)),
      SignalRValue::Float(f) => rmpv::Value::F64(*f),
      SignalRValue::String(s) => rmpv::Value::String(s.clone().into()),
      SignalRValue::Boolean(b) => rmpv::Value::Boolean(*b),
      SignalRValue::Array(a) => rmpv::Value::Array(
        a.iter()
          .map(|v| v.into())
          .collect()
      ),
      SignalRValue::Object(m) => rmpv::Value::Map(
        m.iter()
          .map(|(key, value)| (
            rmpv::Value::String(key.clone().into()),
            value.into()
          ))
          .collect()
      ),
      SignalRValue::Null => rmpv::Value::Nil,
    }
  }
}

impl From<&rmpv::Value> for SignalRValue {
  fn from(value: &rmpv::Value) -> Self {
    match value {
      rmpv::Value::Integer(n) => Self::Integer(n.as_i64().unwrap()),
      rmpv::Value::F64(f) => Self::Float(*f),
      rmpv::Value::F32(f) => Self::Float(*f as f64),
      rmpv::Value::String(s) => Self::String(s.to_string()),
      rmpv::Value::Boolean(b) => Self::Boolean(*b),
      rmpv::Value::Array(a) => Self::Array(
        a.iter()
          .map(|v| v.into())
          .collect()
      ),
      rmpv::Value::Map(m) => Self::Object(
        m.iter()
          .map(|(key, value)| (
            key.to_string(),
            value.into()
          ))
          .collect()
      ),
      rmpv::Value::Nil => Self::Null,
      _ => todo!("idk"),
    }
  }
}