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
            std::vector<char> numb_data;
            int zeros;
            std::tie(numb_data, zeros) = int_to_vec(data_val, sizeof(short));
            out.push_back(EMPConst::INT_16 | (is_negative*128U) | (zeros << 4));
            for (char byte : numb_data) {
                out.push_back(byte);
            }
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
        }
    }
    return out;
}

std::tuple<std::vector<char>, int> int_to_vec(long numb, char size) {
    std::vector<char> out;
    int leading = 0;
    for (int i = 0; i < size; i++) {
        if (numb == 0) {
            leading = size - i;
            break;
        }
        out.push_back(numb & 0xFF);
        numb >>= 8;
    }
    std::reverse(out.begin(), out.end());
    return std::tuple(out, leading);
}