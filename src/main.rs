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
        value::Value::Double(2.34),
        value::Value::Int64(22434),
        value::Value::Int16(23),
        value::Value::Int8(255),
    ]));

    fs::write("encode.emp", &data).expect("Unable to write file");

    let out = decode::decode(&data);
    
    let mut obj = std::collections::HashMap::new();

    obj.insert("index2".to_owned(), value::Value::Float(2.43));
    println!("Original: {}", value::Value::Array(vec![
        value::Value::String("This is a test".to_owned()),
        value::Value::Null,
        value::Value::Boolean(true),
        value::Value::Int32(123),
        value::Value::Object(obj),
        value::Value::Double(2.34),
        value::Value::Int64(22434),
        value::Value::Int16(23),
        value::Value::Int8(255),
    ]));
    match out {
        Ok((v, _)) => println!("Extracted data: {}", v),
        Err(e) => println!("{}", e),
    }
}
