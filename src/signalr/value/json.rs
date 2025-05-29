use super::SignalRValue;

impl Into<serde_json::Value> for &SignalRValue {
  fn into(self) -> serde_json::Value {
    match self {
      SignalRValue::Integer(n) => serde_json::Value::Number(serde_json::Number::from_i128(*n as i128).unwrap()),
      SignalRValue::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap()),
      SignalRValue::String(s) => serde_json::Value::String(s.clone()),
      SignalRValue::Boolean(b) => serde_json::Value::Bool(*b),
      SignalRValue::Array(a) => serde_json::Value::Array(
        a.iter()
          .map(|v| v.into())
          .collect()
      ),
      SignalRValue::Object(m) => serde_json::Value::Object(
        m.iter()
          .map(|(key, value)| (
            key.clone(),
            value.into()
          ))
          .collect()
      ),
      SignalRValue::Null => serde_json::Value::Null,
    }
  }
}

impl From<&serde_json::Value> for SignalRValue {
  fn from(value: &serde_json::Value) -> Self {
    match value {
      serde_json::Value::Number(n) => if n.is_f64() {
        Self::Float(n.as_f64().unwrap())
      } else {
        Self::Integer(n.as_i64().unwrap())
      },
      serde_json::Value::String(s) => Self::String(s.clone()),
      serde_json::Value::Bool(b) => Self::Boolean(*b),
      serde_json::Value::Array(a) => Self::Array(
        a.iter()
          .map(|v| v.into())
          .collect()
      ),
      serde_json::Value::Object(m) => Self::Object(
        m.iter()
          .map(|(key, value)| (
            key.clone(),
            value.into()
          ))
          .collect()
      ),
      serde_json::Value::Null => Self::Null,
    }
  }
}