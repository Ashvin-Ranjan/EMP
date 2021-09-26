use std::env;
use std::string::String;

pub enum ArgumentOptions {
    ReadFromFile(String),
    WriteToFile(String, String),
    FromJSON(String),
    ToJSON(String),
    Version,
    Help,
}

pub fn resolve_arguments(_a: env::Args) -> ArgumentOptions {
    let mut args = _a;
    match args.nth(1) {
        Some(a) => match &a[..] {
            "-r" | "--read" => {
                if let Some(file) = args.nth(0) {
                    return ArgumentOptions::ReadFromFile(file);
                }
                return ArgumentOptions::Help;
            }
            "-w" | "--write" => {
                if let Some(file) = args.nth(0) {
                    if let Some(val) = args.nth(0) {
                        return ArgumentOptions::WriteToFile(file, val);
                    }
                }
                return ArgumentOptions::Help;
            }
            "-fj" | "--from_json" => {
                if let Some(json) = args.nth(0) {
                    return ArgumentOptions::FromJSON(json);
                }
                return ArgumentOptions::Help;
            }
            "-tj" | "--to_json" => {
                if let Some(emp) = args.nth(0) {
                    return ArgumentOptions::ToJSON(emp);
                }
                return ArgumentOptions::Help;
            }
            "-v" | "--version" => return ArgumentOptions::Version,
            _ => return ArgumentOptions::Help,
        },
        None => return ArgumentOptions::Help,
    }
}
