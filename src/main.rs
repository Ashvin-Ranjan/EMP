mod argument;
pub mod constants;
pub mod decode;
pub mod encode;
pub mod errors;
pub mod value;

use argument::ArgumentOptions;
use serde_json;
use std::env;
use std::fs;

fn main() {
  match argument::resolve_arguments(env::args()) {
    ArgumentOptions::ReadFromFile(file) => match fs::read(file) {
      Ok(data) => {
        match decode::decode(&data) {
          Ok((v, _)) => println!("Extracted data: {}", v),
          Err(v) => println!("An error occured while decoding: {}", v),
        };
      }
      Err(_) => println!("Unable to read file (Are you sure it exists?)"),
    },
    ArgumentOptions::FromJSON(json) => match serde_json::from_str::<serde_json::Value>(&json[..]) {
      Ok(json) => {
        println!("{}", value::json::from_json(json))
      }
      Err(_) => println!("Unable to parse JSON Data (Did you put it in quotes?)"),
    },
    ArgumentOptions::Help => {
      println!("┌───────────────────────────────────┐");
      println!("│[E]fficiently [M]anaged [P]ackaging│");
      println!("│               Help:               │");
      println!("│[-r | --read] <filename>: Reads the│");
      println!("│EMP bytecode and prints it out as a│");
      println!("│EMP string.                        │");
      println!("│                                   │");
      println!("│[-fj | --from_json] <json>: Parses │");
      println!("│the JSON data and prints it out as │");
      println!("│an EMP string.                     │");
      println!("│                                   │");
      println!("│NOTE: Make sure your json data is  │");
      println!("│in quotes                          │");
      println!("└───────────────────────────────────┘");
    }
  }
}
