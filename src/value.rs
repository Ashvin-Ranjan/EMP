#[derive(Clone)]
pub enum Value {
  Null,
  Object(std::collections::HashMap<std::string::String, Value>),
  Array(std::vec::Vec<Value>),
  String(std::string::String),
  Bit(bool),
  Boolean(bool),
  Int32(u32),
  Float(f32),
  Double(f64),
  Int64(u64),
  Int16(u16),
  Int8(u8),
}
