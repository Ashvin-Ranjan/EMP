use std::env;
use std::string::String;

pub enum ArgumentOptions {
  ReadFromFile(String),
  FromJSON(String),
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
      "-fj" | "--from_json" => {
        if let Some(json) = args.nth(0) {
          return ArgumentOptions::FromJSON(json);
        }
        return ArgumentOptions::Help;
      }
      "-v" | "--version" => return ArgumentOptions::Version,
      _ => return ArgumentOptions::Help,
    },
    None => return ArgumentOptions::Help,
  }
}
