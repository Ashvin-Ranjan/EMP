pub mod constants;
mod decode;
mod encode;
pub mod value;

use std::fs;

fn main() {
    let mut obj = std::collections::HashMap::new();

    obj.insert("index2".to_owned(), value::Value::Float(2.43));

    // ["This is a test",null,true,123,{"index2":2.43}]
    let data: Vec<u8> = encode::encode(value::Value::Array(vec![
        value::Value::String("This is a test".to_owned()),
        value::Value::Null,
        value::Value::Boolean(true),
        value::Value::Int32(123),
        value::Value::Object(obj),
    ]));

    fs::write("encode.emp", &data).expect("Unable to write file");

    let _out = decode::decode(&data);
}
