# EMP

Efficiently Managed Packaging, NBT but better!

## Using the rust crate

### Rust structure

```rs
pub enum Value {
  Null,
  Object(std::collections::HashMap<std::string::String, Value>),
  Array(std::vec::Vec<Value>),
  String(std::string::String),
  Bit(bool),
  Boolean(bool),
  Int32(i32),
  Float(f32),
  Double(f64),
  Int64(i64),
  Int16(i16),
  Int8(i8),
}
```

### Encoding

Using `emp::encode::encode` you can pass in a `emp::value::Value` and get an `std::vec::Vec<u8>` back, this will encode the data in the `Value` into the bytes that you can then write to a file.

### Decoding

Using `emp::decode::decode` you can pass in a `&[u8]` and get an `Result<emp::value::Value, emp::errors::DecodeError>` in return.

`DecodeError` is an enum that is as so:

```rs
pub enum DecodeError {
  UnexpectedByte(u8, u64),
  EOFError,
  UnmatchedKey(std::string::String),
  StringDecodeError(std::str::Utf8Error),
}
```

You can also use `emp::decode::decode_safe` and pass in the same data to decode the data in the same way but if there is an `Err` it instead returns `emp::value::Value::Null`.

### Parsing

Using the `emp::value::parse::from_str` function you can pass in a `&str` to convert it into a `Result<emp::value::Value, emp::errors::ParseError>`. Alternatively you can use `emp::value::parse::from_str_safe` and pass in the same thing to get a `emp::value::Value`, if an error is encounted it returns a `emp::value::Value::Null` instead.

### JSON Compatability

#### Conversion

This crate is compatible with `serde_json`, by using `emp::value::json::from_json` you can convert a `serde_json::Value` to an `emp::value::Value`, vice versa for `emp::value::json::to_json`.

#### Encoding

Using `emp::encode::json::encode_json` you can encode a `serde_json::Value` directly into `emp` bytecode.

#### Decoding

Using `emp::decode::json::decode_json` you can decode `emp` bytecode directly into a `serde_json::Value`, this uses `decode_safe` rather than `decode`.
You can also use `emp::decode::json::decode_json_unsafe` to get the error instead of a `serde_json::Value::Null`

## The Command Line Utility

You are able to run it using `emp`:
```
┌───────────────────────────────────┐
│[E]fficiently [M]anaged [P]ackaging│
│               Help:               │
│                                   │
│[-r | --read] <filename>: Reads the│
│EMP bytecode and prints it out as a│
│EMP string.                        │
│                                   │
│[-w | --write] <filename> <emp>:   │
|Writes the EMP data into the file  │
│as EMP bytecode.                   │
│                                   │
│NOTE: Make sure your EMP data is in│
│quotes.                            │
│                                   │
│[-fj | --from_json] <json>: Parses │
│the JSON data and prints it out as │
│an EMP string.                     │
│                                   │
│NOTE: Make sure your json data is  │
│in quotes.                         │
│                                   │
│[-tj | --to_json] <emp>: Parses the│
│EMP data and prints it out as a    │
│json string.                       │
│                                   │
│NOTE: Make sure your EMP data is in│
│quotes.                            │
│                                   │
│[-v | --version]: Prints out the   │
│version of EMP you are using       │
└───────────────────────────────────┘
```