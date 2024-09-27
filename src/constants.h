#ifndef CONST_HEADER_H
#define CONST_HEADER_H

#include <cstdint>

namespace EMPConst {
    const uint8_t DICTIONARY_START = 0b00000000;
    const uint8_t DICTIONARY_END = 0b00000001;
    const uint8_t ARRAY_START = 0b00000010;
    const uint8_t ARRAY_END = 0b00000011;
    const uint8_t STRING = 0b00000100;
    const uint8_t NONE = 0b00000101;
    const uint8_t BIT = 0b00000111;
    const uint8_t BOOLEAN = 0b00001000;
    const uint8_t INT_32 = 0b00001001;
    const uint8_t FLOAT = 0b00001010;
    const uint8_t DOUBLE = 0b00001011;
    const uint8_t INT_64 = 0b00001100;
    const uint8_t INT_16 = 0b00001101;
    const uint8_t INT_8 = 0b00001110;

    const char EMP_QUOTE = '"';
    const char EMP_OPEN_BRACE = '[';
    const char EMP_CLOSE_BRACE = ']';
    const char EMP_OPEN_BRACKET = '{';
    const char EMP_CLOSE_BRACKET = '}';
    const char EMP_COMMA = ',';
    const char EMP_COLON = ':';
    const char EMP_NULL[5] = "null";
    const char EMP_FALSE[6] = "false";
    const char EMP_TRUE[5] = "true";
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
        EMP_BIT, EMP_BYTE, EMP_LONG, EMP_SHORT, EMP_FLOAT, EMP_DOUBLE,
    };
}

#endif