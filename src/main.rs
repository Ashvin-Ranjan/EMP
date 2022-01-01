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
    match fs::read_to_string("encode.json") {
        Ok(data) => match serde_json::from_str::<serde_json::Value>(&data[..]) {
            Ok(json) => {
                fs::write("encode.emp", &encode::encode(value::json::from_json(json)))
                    .expect("oop");
            }
            Err(_) => println!("Unable to parse JSON Data (Did you put it in quotes?)"),
        },
        Err(_) => println!("Unable to read file (Are you sure it exists?)"),
    }
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
        ArgumentOptions::WriteToFile(file, val) => {
            let data = match value::parse::from_str(&val) {
                Ok(emp) => emp,
                Err(e) => {
                    println!(
                        "Unable to parse EMP Data (Did you put it in quotes?): {}",
                        e
                    );
                    return;
                }
            };

            fs::write(file, &encode::encode(data))
                .expect("Unable to write to file (Do you have permission?)");
        }
        ArgumentOptions::FromJSON(json) => {
            match serde_json::from_str::<serde_json::Value>(&json[..]) {
                Ok(json) => {
                    println!("{}", value::json::from_json(json))
                }
                Err(_) => println!("Unable to parse JSON Data (Did you put it in quotes?)"),
            }
        }
        ArgumentOptions::ToJSON(emp) => match value::parse::from_str(&emp) {
            Ok(emp) => {
                println!("{}", value::json::to_json(emp))
            }
            Err(e) => println!(
                "Unable to parse EMP Data (Did you put it in quotes?): {}",
                e
            ),
        },
        ArgumentOptions::Version => println!("EMP {}", env!("CARGO_PKG_VERSION")),
        ArgumentOptions::Help => {
            println!("┌───────────────────────────────────┐");
            println!("│[E]fficiently [M]anaged [P]ackaging│");
            println!("│               Help:               │");
            println!("│                                   │");
            println!("│[-r | --read] <filename>: Reads the│");
            println!("│EMP bytecode and prints it out as a│");
            println!("│EMP string.                        │");
            println!("│                                   │");
            println!("│[-w | --write] <filename> <emp>:   │");
            println!("|Writes the EMP data into the file  │");
            println!("│as EMP bytecode.                   │");
            println!("│                                   │");
            println!("│NOTE: Make sure your EMP data is in│");
            println!("│quotes.                            │");
            println!("│                                   │");
            println!("│[-fj | --from_json] <json>: Parses │");
            println!("│the JSON data and prints it out as │");
            println!("│an EMP string.                     │");
            println!("│                                   │");
            println!("│NOTE: Make sure your json data is  │");
            println!("│in quotes.                         │");
            println!("│                                   │");
            println!("│[-tj | --to_json] <emp>: Parses the│");
            println!("│EMP data and prints it out as a    │");
            println!("│json string.                       │");
            println!("│                                   │");
            println!("│NOTE: Make sure your EMP data is in│");
            println!("│quotes.                            │");
            println!("│                                   │");
            println!("│[-v | --version]: Prints out the   │");
            println!("│version of EMP you are using       │");
            println!("└───────────────────────────────────┘");
        }
    }
}
