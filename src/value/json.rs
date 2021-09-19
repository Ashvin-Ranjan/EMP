use crate::value;
use serde_json;
use std::collections::HashMap;

pub fn from_json(val: serde_json::Value) -> value::Value {
    match val {
        serde_json::Value::Null => return value::Value::Null,
        serde_json::Value::Bool(b) => return value::Value::Boolean(b),
        serde_json::Value::Number(n) => {
            match n.as_f64() {
                Some(doub) => return value::Value::Double(doub),
                None => {
                    match n.as_u64() {
                        Some(numb) => {
                            if numb < 2 {
                                return value::Value::Bit(numb == 1);
                            }

                            if numb < u8::MAX as u64 {
                                return value::Value::Int8(numb as u8);
                            }

                            if numb < u16::MAX  as u64 {
                                return value::Value::Int16(numb as u16);
                            }

                            if numb < u32::MAX as u64 {
                                return value::Value::Int32(numb as u32);
                            }

                            return value::Value::Int64(numb);
                        }
                        None => return value::Value::Null,
                    }
                }
            }


        }
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
                out.insert(key.clone(), match o.get(key) {
                    Some(s) => from_json(s.clone()),
                    None => value::Value::Null,
                  });
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
                out.insert(key.clone(), match o.get(key) {
                    Some(s) => to_json(s.clone()),
                    None => serde_json::Value::Null,
                  });
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
        value::Value::Bit(i) => return serde_json::json!(if i { 1 } else { 0 } ),
        value::Value::Boolean(b) => return serde_json::json!(b),
        value::Value::Int32(i) => return serde_json::json!(i),
        value::Value::Float(f) => return serde_json::json!(f),
        value::Value::Double(d) => return serde_json::json!(d),
        value::Value::Int64(i) => return serde_json::json!(i),
        value::Value::Int16(i) => return serde_json::json!(i),
        value::Value::Int8(i) => return serde_json::json!(i),
    }
}