#include "structure.h"
#include "encode.h"
#include "constants.h"
#include <tuple>

std::vector<char> encode(EMPData data) {
    std::vector<char> out;
    switch (data.type) {
        case INT_8:
        {
            char data_val = *(char*)data.data;
            if ((data_val & 0xf) == data_val) {
                out.push_back((data_val << 4) | EMPConst::INT_8);
            } else {
                out.push_back(EMPConst::INT_8);
                out.push_back(data_val);
            }
            break;
        }
        case NONE:
            out.push_back(EMPConst::NONE);
            break;
        case BOOL:
        {
            bool data_val = *(bool*)data.data;
            out.push_back(EMPConst::BOOLEAN | (data_val*128U));
            break;
        }
        case INT_16: 
        {
            long data_val = (long)*(short*)data.data;
            bool is_negative = data_val < 0;
            data_val *= is_negative ? -1 : 1;
            std::vector<char> numb_data = int_to_vec(data_val, sizeof(short));
            out.push_back(EMPConst::INT_16 | (is_negative*128U) | (numb_data.back() << 4));
            numb_data.pop_back();
            for (char byte : numb_data) {
                out.push_back(byte);
            }
            break;
        }
        case ARRAY:
        {
            std::vector<EMPData> data_val = *(std::vector<EMPData>*)data.data;
            bool is_short = data_val.size() < 0xf;
            out.push_back(EMPConst::ARRAY_START | (is_short*(data_val.size() << 4)));
            for (EMPData d : data_val) {
                std::vector<char> d_encoded = encode(d);
                out.insert(out.end(), d_encoded.begin(), d_encoded.end());
            }
            if (!is_short) {
                out.push_back(EMPConst::ARRAY_END);
            }
            break;
        }
        case FLOAT:
        {
            float data_val = *(float*)data.data;
            std::vector<char> byte_data = float_to_vec(data_val);
            out.push_back(EMPConst::FLOAT | byte_data.back());
            byte_data.pop_back();
            out.insert(out.end(), byte_data.begin(), byte_data.end());
            break;
        }
    }
    return out;
}

// Last item in the vector is data to be put in the identifier
std::vector<char> int_to_vec(long numb, char size) {
    std::vector<char> out;
    char leading = 0;
    for (int i = 0; i < size; i++) {
        if (numb == 0) {
            leading = size - i;
            break;
        }
        out.push_back(numb & 0xFF);
        numb >>= 8;
    }
    std::reverse(out.begin(), out.end());
    out.push_back(leading);
    return out;
}

// Last item in the vector is data to be put in the identifier 
// Float format:
// [Sign (1 bit)] [Exponent (8 bits)] [Mantissa (23 bits)]
std::vector<char> float_to_vec(float numb) {
    std::vector<char> out;
    long bit_data = *(long*)&numb; // Evil floating point bit level hacking
    long exponent = (bit_data >> 23) & 0xff;
    long sign = bit_data >> 31;
    long mantissa = bit_data & 0x7fffff;
    if (exponent < 0x10) {
        long push_data = sign << 23 | mantissa;
        for (int i = 0; i < 3; i++) {
            out.push_back(push_data & 0xFF);
            push_data >>= 8;
        }
        std::reverse(out.begin(), out.end());
        out.push_back((char)exponent << 4);
    } else {
        for (int i = 0; i < 4; i++) {
            out.push_back(bit_data & 0xFF);
            bit_data >>= 8;
        }
        std::reverse(out.begin(), out.end());
        out.push_back(0);
    }
    return out;
}