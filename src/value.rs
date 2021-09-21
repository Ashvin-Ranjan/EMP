pub mod json;

use std::fmt;

#[derive(Clone)]
pub enum Value {
  Null,
  Object(std::collections::HashMap<std::string::String, Value>),
  Array(std::vec::Vec<Value>),
  String(std::string::String),
  Bit(bool),
  Boolean(bool),
  Int32(i32),
  Float(f32),
  Double(f64),
  Int64(i64),
  Int16(i16),
  Int8(i8),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", display(self.clone()))
  }
}

fn display(val: Value) -> String {
  match val {
    Value::String(s) => return s,
    Value::Bit(i) => return (if i { "1" } else { "0" }).to_owned(),
    Value::Boolean(i) => return (if i { "true" } else { "false" }).to_owned(),
    Value::Int32(i) => return format!("{}", i),
    Value::Int64(i) => return format!("{}l", i),
    Value::Int16(i) => return format!("{}s", i),
    Value::Int8(i) => return format!("{}B", i),
    Value::Float(i) => return format!("{}f", i),
    Value::Double(i) => return format!("{}d", i),
    Value::Array(a) => {
      let mut arr = vec![];
      for v in a {
        arr.push(display(v));
      }

      return format!("[{}]", arr.join(", "));
    }
    Value::Object(o) => {
      let mut out = String::from("");

      for key in o.keys() {
        out = format!(
          "{}, \"{}\": {}",
          out,
          key,
          display(match o.get(key) {
            Some(s) => s.clone(),
            None => Value::Null,
          })
        );
      }

      return format!("{{{}}}", out[2..].to_owned());
    }
    Value::Null => return "null".to_owned(),
  }
}
