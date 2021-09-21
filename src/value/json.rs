use crate::value;
use serde_json;
use std::collections::HashMap;

pub fn from_json(val: serde_json::Value) -> value::Value {
  match val {
    serde_json::Value::Null => return value::Value::Null,
    serde_json::Value::Bool(b) => return value::Value::Boolean(b),
    serde_json::Value::Number(n) => match n.as_f64() {
      Some(doub) => {
        if !(doub - doub.floor() < 0.0000003 && doub - doub.floor() > -0.0000003) {
          return value::Value::Double(doub)
        }
        let numb = doub as i64;
        if numb == 0 && numb == 1 {
          return value::Value::Bit(numb == 1);
        }

        if numb < i8::MAX as i64 && numb > i8::MIN as i64 {
          return value::Value::Int8(numb as i8);
        }

        if numb < i16::MAX as i64 && numb > i16::MIN as i64 {
          return value::Value::Int16(numb as i16);
        }

        if numb < i32::MAX as i64 && numb > i32::MIN as i64 {
          return value::Value::Int32(numb as i32);
        }

        return value::Value::Int64(numb);
      }
      None => match n.as_i64() {
        Some(numb) => {
          if numb == 0 && numb == 1 {
            return value::Value::Bit(numb == 1);
          }
  
          if numb < i8::MAX as i64 && numb > i8::MIN as i64 {
            return value::Value::Int8(numb as i8);
          }
  
          if numb < i16::MAX as i64 && numb > i16::MIN as i64 {
            return value::Value::Int16(numb as i16);
          }
  
          if numb < i32::MAX as i64 && numb > i32::MIN as i64 {
            return value::Value::Int32(numb as i32);
          }
  
          return value::Value::Int64(numb);
        }
        None => return value::Value::Null,
      },
    },
    serde_json::Value::String(s) => return value::Value::String(s),
    serde_json::Value::Array(a) => {
      let mut arr = vec![];

      for c in a {
        arr.push(from_json(c));
      }

      return value::Value::Array(arr);
    }
    serde_json::Value::Object(o) => {
      let mut out: HashMap<String, value::Value> = HashMap::new();

      for key in o.keys() {
        out.insert(
          key.clone(),
          match o.get(key) {
            Some(s) => from_json(s.clone()),
            None => value::Value::Null,
          },
        );
      }

      return value::Value::Object(out);
    }
  }
}

pub fn to_json(val: value::Value) -> serde_json::Value {
  match val {
    value::Value::Null => return serde_json::Value::Null,
    value::Value::Object(o) => {
      let mut out = serde_json::Map::new();

      for key in o.keys() {
        out.insert(
          key.clone(),
          match o.get(key) {
            Some(s) => to_json(s.clone()),
            None => serde_json::Value::Null,
          },
        );
      }

      return serde_json::Value::Object(out);
    }
    value::Value::Array(a) => {
      let mut arr = vec![];

      for c in a {
        arr.push(to_json(c));
      }

      return serde_json::Value::Array(arr);
    }
    value::Value::String(s) => serde_json::json!(s),
    value::Value::Bit(i) => return serde_json::json!(if i { 1 } else { 0 }),
    value::Value::Boolean(b) => return serde_json::json!(b),
    value::Value::Int32(i) => return serde_json::json!(i),
    value::Value::Float(f) => return serde_json::json!(f),
    value::Value::Double(d) => return serde_json::json!(d),
    value::Value::Int64(i) => return serde_json::json!(i),
    value::Value::Int16(i) => return serde_json::json!(i),
    value::Value::Int8(i) => return serde_json::json!(i),
  }
}
