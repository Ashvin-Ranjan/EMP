//! Constants for internal use

pub const DICTIONARY_START: u8 = 0b00000000;
pub const DICTIONARY_END: u8 = 0b00000001;
pub const ARRAY_START: u8 = 0b00000010;
pub const ARRAY_END: u8 = 0b00000011;
pub const STRING: u8 = 0b00000100;
pub const NULL: u8 = 0b00000101;
pub const BIT: u8 = 0b00000111;
pub const BOOLEAN: u8 = 0b00001000;
pub const INT_32: u8 = 0b00001001;
pub const FLOAT: u8 = 0b00001010;
pub const DOUBLE: u8 = 0b00001011;
pub const INT_64: u8 = 0b00001100;
pub const INT_16: u8 = 0b00001101;
pub const INT_8: u8 = 0b00001110;

pub const EMP_QUOTE: char = '"';
pub const EMP_OPEN_BRACE: char = '[';
pub const EMP_CLOSE_BRACE: char = ']';
pub const EMP_OPEN_BRACKET: char = '{';
pub const EMP_CLOSE_BRACKET: char = '}';
pub const EMP_COMMA: char = ',';
pub const EMP_COLON: char = ':';
pub const EMP_NULL: &str = "null";
pub const EMP_FALSE: &str = "false";
pub const EMP_TRUE: &str = "true";
pub const EMP_BIT: char = 'b';
pub const EMP_BYTE: char = 'B';
pub const EMP_LONG: char = 'l';
pub const EMP_SHORT: char = 's';
pub const EMP_FLOAT: char = 'f';
pub const EMP_DOUBLE: char = 'd';
pub const EMP_PERIOD: char = '.';
pub const EMP_ESCAPE: char = '\\';
pub const EMP_WHITESPACE: [char; 4] = [' ', '\t', '\n', '\r'];
pub const EMP_CONTROL: [char; 6] = [
    EMP_COMMA,
    EMP_COLON,
    EMP_OPEN_BRACE,
    EMP_OPEN_BRACKET,
    EMP_CLOSE_BRACE,
    EMP_CLOSE_BRACKET,
];
pub const EMP_NUMB_SIGN: [char; 6] = [
    EMP_BIT, EMP_BYTE, EMP_LONG, EMP_SHORT, EMP_FLOAT, EMP_DOUBLE,
];
