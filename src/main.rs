pub mod constants;
pub mod decode;
pub mod encode;
pub mod errors;
pub mod value;

use std::fs;
use std::io::Read;

fn main() {
  let mut obj = std::collections::HashMap::new();

  obj.insert("index2".to_owned(), value::Value::Float(2.43));

  if let Ok(readata) = fs::read_to_string("./test.json") {
    let data = encode::json::encode_json(serde_json::from_str(&readata).unwrap());

    fs::write("encode.emp", &data).expect("Unable to write file");

    let mut d = std::vec::Vec::new();

    let mut f = fs::File::open("encode.emp").expect("Unable to read file");

    f.read_to_end(&mut d).expect("Cannot read");

    let out = decode::json::decode_json_unsafe(&d);

    match out {
      Ok(o) => println!("Extracted data: {}", o),
      Err(e) => println!("Error: {}", e),
    }
  }
}
