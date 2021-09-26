use crate::constants;
use crate::errors::ParseError;
use crate::value::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;

#[macro_use]
mod macros;

const DIGITS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', '-'];

fn lex_num(mut string: &str) -> Result<(Option<String>, &str), ParseError> {
    let mut emp_numb = String::from("");

    if !DIGITS.contains(&string.chars().next().unwrap()) {
        return Ok((None, string));
    }

    while string.len() != 0 {
        if !DIGITS.contains(&string.chars().next().unwrap()) {
            if constants::EMP_NUMB_SIGN.contains(&string.chars().next().unwrap()) {
                emp_numb.push_str(&string.chars().next().unwrap().to_string());
                string = &string[1..];
            }
            return Ok((Some(emp_numb), string));
        }

        emp_numb.push_str(&string.chars().next().unwrap().to_string());
        string = &string[1..];
    }

    return Ok((Some(emp_numb), string));
}

fn lex_null(string: &str) -> (Option<String>, &str) {
    if string.len() < 4 {
        return (None, string);
    }

    if &string[0..4] == "null" {
        return (Some("null".to_owned()), &string[4..]);
    }

    return (None, string);
}

fn lex_string(mut string: &str) -> Result<(Option<String>, &str), ParseError> {
    let mut emp_string = String::from("\"");

    if string.chars().next().unwrap() != constants::EMP_QUOTE {
        return Ok((None, string));
    }

    string = &string[1..];

    while string.len() != 0 {
        if string.chars().next().unwrap() == constants::EMP_QUOTE
            && emp_string.chars().last().unwrap() == constants::EMP_ESCAPE
        {
            emp_string = emp_string[..emp_string.len() - 1].to_owned();
            emp_string.push_str("\"");
            string = &string[1..];
        } else if string.chars().next().unwrap() == constants::EMP_QUOTE {
            emp_string.push_str("\"");
            return Ok((Some(emp_string), &string[1..]));
        } else {
            emp_string.push_str(&string.chars().next().unwrap().to_string());
            string = &string[1..];
        }
    }

    return Err(ParseError::EOFError);
}

fn lex_bool(string: &str) -> (Option<String>, &str) {
    if string.len() < 4 {
        return (None, string);
    }

    if &string[0..4] == "true" {
        return (Some("true".to_owned()), &string[4..]);
    }

    if string.len() < 5 {
        return (None, string);
    }

    if &string[0..5] == "false" {
        return (Some("false".to_owned()), &string[5..]);
    }

    return (None, string);
}

pub fn lex(mut string: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens: Vec<String> = vec![];

    while string.len() != 0 {
        match lex_num(string) {
            Ok((t, s)) => {
                string = s;
                if let Some(token) = t {
                    tokens.push(token);
                    continue;
                }
            }
            Err(e) => return Err(e),
        }

        match lex_null(string) {
            (Some(token), s) => {
                tokens.push(token);
                string = s;
                continue;
            }
            (None, _) => {}
        }

        match lex_bool(string) {
            (Some(token), s) => {
                tokens.push(token);
                string = s;
                continue;
            }
            (None, _) => {}
        }

        match lex_string(string) {
            Ok((t, s)) => {
                string = s;
                if let Some(token) = t {
                    tokens.push(token);
                    continue;
                }
            }
            Err(e) => return Err(e),
        }

        if constants::EMP_WHITESPACE.contains(&string.chars().next().unwrap()) {
            string = &string[1..];
            continue;
        }
        if constants::EMP_CONTROL.contains(&string.chars().next().unwrap()) {
            tokens.push(String::from(string.chars().next().unwrap()));
            string = &string[1..];
            continue;
        }

        return Err(ParseError::UnexpectedCharacterError(
            string.chars().next().unwrap(),
        ));
    }

    return Ok(tokens);
}

fn parse_array(mut tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    let mut values: Vec<Value> = vec![];

    if tokens[0] != String::from(constants::EMP_OPEN_BRACE) {
        return Ok((None, tokens));
    }

    tokens = &tokens[1..];

    while tokens.len() != 0 {
        if tokens[0] == String::from(constants::EMP_CLOSE_BRACE) {
            return Ok((Some(Value::Array(values)), &tokens[1..]));
        }

        match parse(tokens) {
            Ok((val, tok)) => {
                tokens = tok;
                values.push(val);
            }
            Err(e) => return Err(e),
        }

        if tokens[0] != "," && tokens[0] != constants::EMP_CLOSE_BRACE.to_string() {
            return Err(ParseError::UnexpectedTokenError(tokens[0].clone()));
        }
        if tokens[0] == "," {
            tokens = &tokens[1..];
        }
    }

    return Err(ParseError::EOFError);
}

fn parse_object(mut tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    let mut values: HashMap<String, Value> = HashMap::new();

    if tokens[0] != String::from(constants::EMP_OPEN_BRACKET) {
        return Ok((None, tokens));
    }

    tokens = &tokens[1..];

    while tokens.len() != 0 {
        if tokens[0] == String::from(constants::EMP_CLOSE_BRACKET) {
            return Ok((Some(Value::Object(values)), &tokens[1..]));
        }

        let key;

        match parse(tokens) {
            Ok((val, tok)) => {
                tokens = tok;
                match val {
                    Value::String(s) => key = s,
                    _ => return Err(ParseError::InvalidKeyError(val)),
                }
            }
            Err(e) => return Err(e),
        }

        if tokens[0] != ":" {
            return Err(ParseError::UnexpectedTokenError(tokens[0].clone()));
        }
        tokens = &tokens[1..];

        match parse(tokens) {
            Ok((val, tok)) => {
                tokens = tok;
                values.insert(key, val);
            }
            Err(e) => return Err(e),
        }

        if tokens[0] != "," && tokens[0] != constants::EMP_CLOSE_BRACKET.to_string() {
            return Err(ParseError::UnexpectedTokenError(tokens[0].clone()));
        }
        if tokens[0] == "," {
            tokens = &tokens[1..];
        }
    }

    return Err(ParseError::EOFError);
}

fn parse_null(tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    if tokens[0] == "null" {
        return Ok((Some(Value::Null), &tokens[1..]));
    }

    return Ok((None, tokens));
}

fn parse_bool(tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    if tokens[0] == "true" {
        return Ok((Some(Value::Boolean(true)), &tokens[1..]));
    }

    if tokens[0] == "false" {
        return Ok((Some(Value::Boolean(false)), &tokens[1..]));
    }

    return Ok((None, tokens));
}

fn parse_string(tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    if tokens[0].chars().next().unwrap() != constants::EMP_QUOTE {
        return Ok((None, tokens));
    }

    return Ok((
        Some(Value::String(tokens[0][1..tokens[0].len() - 1].to_owned())),
        &tokens[1..],
    ));
}

fn parse_number(tokens: &[String]) -> Result<(Option<Value>, &[String]), ParseError> {
    if !DIGITS.contains(&tokens[0].chars().next().unwrap()) {
        return Ok((None, tokens));
    }

    match tokens[0].chars().last().unwrap() {
        constants::EMP_BIT => {
            return Ok((
                Some(Value::Bit(tokens[0].chars().next().unwrap() == '1')),
                &tokens[1..],
            ))
        }
        constants::EMP_BYTE => match i8::from_str(&tokens[0][..tokens[0].len() - 1]) {
            Ok(n) => return Ok((Some(Value::Int8(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
        constants::EMP_LONG => match i64::from_str(&tokens[0][..tokens[0].len() - 1]) {
            Ok(n) => return Ok((Some(Value::Int64(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
        constants::EMP_SHORT => match i16::from_str(&tokens[0][..tokens[0].len() - 1]) {
            Ok(n) => return Ok((Some(Value::Int16(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
        constants::EMP_FLOAT => match f32::from_str(&tokens[0][..tokens[0].len() - 1]) {
            Ok(n) => return Ok((Some(Value::Float(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
        constants::EMP_DOUBLE => match f64::from_str(&tokens[0][..tokens[0].len() - 1]) {
            Ok(n) => return Ok((Some(Value::Double(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
        _ => match i32::from_str(&tokens[0]) {
            Ok(n) => return Ok((Some(Value::Int32(n)), &tokens[1..])),
            Err(_) => {
                return Err(ParseError::InvalidNumberError(
                    tokens[0].chars().last().unwrap(),
                ))
            }
        },
    }
}

pub fn parse(tokens: &[String]) -> Result<(Value, &[String]), ParseError> {
    try_parse!(parse_array, tokens);
    try_parse!(parse_object, tokens);
    try_parse!(parse_null, tokens);
    try_parse!(parse_bool, tokens);
    try_parse!(parse_string, tokens);
    try_parse!(parse_number, tokens);

    return Err(ParseError::UnexpectedTokenError(tokens[0].clone()));
}

pub fn from_str(string: &str) -> Result<Value, ParseError> {
    match lex(string) {
        Ok(tok) => match parse(&tok) {
            Ok((val, _)) => Ok(val),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub fn from_str_safe(string: &str) -> Value {
    match from_str(string) {
        Ok(o) => o,
        Err(_) => Value::Null,
    }
}