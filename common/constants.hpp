#pragma once

// Constants for internal use

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