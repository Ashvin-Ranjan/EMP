#include <vector>

#include "..\common\constants.hpp"
#include "types.hpp"

using namespace std;

// Nulls just simply return 101
vector<unsigned char> EMPNullNode::encode()
{
    vector<unsigned char> out;
    out.push_back(NONE);
    return out;
}

// Bits and Booleans return their identification code with the 4th bit
// indicating the value
vector<unsigned char> EMPBitNode::encode()
{
    vector<unsigned char> out;
    out.push_back(BIT | value << 4);
    return out;
}

vector<unsigned char> EMPBooleanNode::encode()
{
    vector<unsigned char> out;
    out.push_back(BOOLEAN | value << 4);
    return out;
}

// Strings store their length in the first 4 bits if possible to remove the
// closing byte, otherwise it uses a closing byte.
vector<unsigned char> EMPStringNode::encode()
{
    vector<unsigned char> out;
    out.push_back(STRING | (value.length() <= 0xf && value.length() > 0 ? value.length() : 0) << 4);
    copy(value.begin(), value.end(), back_inserter(out));
    if (!(value.length() <= 0xf && value.length() > 0))
        out.push_back(STRING);
    return out;
}

// All ints store the amount of leading 0 bytes in the top 4 bits
//
// TODO: Make heading + tailing bytes optimization
// vector<unsigned char> EMPInt64Node::encode()
// {
//     vector<unsigned char> out;
//     out.push_back(STRING | (value.length() <= 0xff && value.length() > 0 ? value.length() : 0) << 4);
//     copy(value.begin(), value.end(), back_inserter(out));
//     if (!(value.length() <= 0xff && value.length() > 0))
//         out.push_back(STRING);
//     return out;
// }