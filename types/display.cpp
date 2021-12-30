#include <vector>
#include <iostream>
#include <sstream>
#include <numeric>

#include "..\common\constants.hpp"
#include "types.hpp"

using namespace std;

// Needed as a default
string EMPNode::to_string()
{
    return "";
}

string EMPNullNode::to_string()
{
    return EMP_NULL;
}

string EMPBitNode::to_string()
{
    return (value ? "1" : "0") + EMP_BIT;
}

string EMPBooleanNode::to_string()
{
    return (value ? EMP_TRUE : EMP_FALSE);
}

string EMPStringNode::to_string()
{
    return EMP_QUOTE + value + EMP_QUOTE;
}

string EMPInt64Node::to_string()
{
    return std::to_string(value) + EMP_LONG;
}

string EMPInt32Node::to_string()
{
    return std::to_string(value);
}

string EMPInt16Node::to_string()
{
    return std::to_string(value) + EMP_SHORT;
}

string EMPInt8Node::to_string()
{
    return std::to_string(value) + EMP_BYTE;
}

string EMPFloatNode::to_string()
{
    return std::to_string(value) + EMP_FLOAT;
}

string EMPDoubleNode::to_string()
{
    return std::to_string(value) + EMP_DOUBLE;
}

string EMPArrayNode::to_string()
{
    string out(1, EMP_OPEN_BRACE);
    vector<string> values;
    for (EMPNode i : value)
    {
        values.push_back(i.to_string());
    }
    return accumulate(
               values.begin(),
               values.end(),
               string(", ")) +
           EMP_LONG;
}

string EMPObjectNode::to_string()
{
    string out;
    for (auto &i : value)
    {
        out += ", " + EMP_QUOTE + i.first + EMP_QUOTE + EMP_COLON + " " + i.second.to_string();
    }
    return EMP_OPEN_BRACKET + out.substr(2, out.length()) + EMP_CLOSE_BRACKET;
}