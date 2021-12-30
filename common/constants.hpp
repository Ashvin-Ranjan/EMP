#pragma once

// Constants for internal use

// Byte constants
const unsigned char DICTIONARY_START = 0b00000000;
const unsigned char DICTIONARY_END = 0b00000001;
const unsigned char ARRAY_START = 0b00000010;
const unsigned char ARRAY_END = 0b00000011;
const unsigned char STRING = 0b00000100;
// Cannot name it NULL due to compiler reasons
const unsigned char NONE = 0b00000101;
const unsigned char BIT = 0b00000111;
const unsigned char BOOLEAN = 0b00001000;
const unsigned char INT_32 = 0b00001001;
const unsigned char FLOAT = 0b00001010;
const unsigned char DOUBLE = 0b00001011;
const unsigned char INT_64 = 0b00001100;
const unsigned char INT_16 = 0b00001101;
const unsigned char INT_8 = 0b00001110;

// String constants
const char EMP_QUOTE = '"';
const char EMP_OPEN_BRACE = '[';
const char EMP_CLOSE_BRACE = ']';
const char EMP_OPEN_BRACKET = '{';
const char EMP_CLOSE_BRACKET = '}';
const char EMP_COMMA = ',';
const char EMP_COLON = ':';
const std::string EMP_NULL = "null";
const std::string EMP_FALSE = "false";
const std::string EMP_TRUE = "true";
const char EMP_BIT = 'b';
const char EMP_BYTE = 'B';
const char EMP_LONG = 'l';
const char EMP_SHORT = 's';
const char EMP_FLOAT = 'f';
const char EMP_DOUBLE = 'd';
const char EMP_PERIOD = '.';
const char EMP_ESCAPE = '\\';
const char EMP_WHITESPACE[4] = {' ', '\t', '\n', '\r'};
const char EMP_CONTROL[6] = {
    EMP_COMMA,
    EMP_COLON,
    EMP_OPEN_BRACE,
    EMP_OPEN_BRACKET,
    EMP_CLOSE_BRACE,
    EMP_CLOSE_BRACKET,
};
const char EMP_NUMB_SIGN[6] = {
    EMP_BIT,
    EMP_BYTE,
    EMP_LONG,
    EMP_SHORT,
    EMP_FLOAT,
    EMP_DOUBLE,
};