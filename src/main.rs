pub mod constants;
pub mod decode;
pub mod encode;
pub mod errors;
pub mod value;

use std::fs;

fn main() {
  let mut obj = std::collections::HashMap::new();

  obj.insert("index2".to_owned(), value::Value::Float(2.43));

  if let Ok(readata) = fs::read_to_string("./test.json") {
    let data = encode::json::encode_json(serde_json::from_str(&readata).unwrap());

    fs::write("encode.emp", &data).expect("Unable to write file");

    let out = decode::json::decode_json(&data);

    println!("Extracted data: {}", out)
  }
}
