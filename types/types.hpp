#pragma once

#include <vector>
#include <map>

// EMPNodes are the base class for everything to branch off of
//
// To make it polymorphic, it needs a `virtual` function, accorting to:
// https://stackoverflow.com/questions/15114093/getting-source-type-is-not-polymorphic-when-trying-to-use-dynamic-cast
// The desctructor is the best one to add if there is nothing else to add.
class EMPNode
{
public:
    virtual ~EMPNode(){};
    std::vector<unsigned char> encode();
};

// EMPNullNodes Nodes are `null` values
//
// Since they do not contain any inherent data, they are an empty class that
// inherits from EMPNode. However they do need a constructor
class EMPNullNode : public EMPNode
{
public:
    std::vector<unsigned char> encode();
    EMPNullNode(){};
};

// EMPStringNodes are std::string values
class EMPStringNode : public EMPNode
{
public:
    std::string value;
    std::vector<unsigned char> encode();

    EMPStringNode(std::string val)
    {
        value = val;
    }
};

// EMPBitNodes are bit values
//
// Note that they take in a boolean, this is because C++ does not contain a
// standard bit type, so boolean is the other best bet with:
//
// false: 0
// true: 1
class EMPBitNode : public EMPNode
{
public:
    bool value;
    std::vector<unsigned char> encode();

    EMPBitNode(bool val)
    {
        value = val;
    }
};

// EMPBooleanNodes are bool values
//
// Due to reasons booleans still take up the same space in memory as a char
// which does make the EMPBitNode and EMPBooleanNode slightly inefficient
// but this does not quite matter, as they only need something with two
// definate states to be passed in
class EMPBooleanNode : public EMPNode
{
public:
    bool value;
    std::vector<unsigned char> encode();

    EMPBooleanNode(bool val)
    {
        value = val;
    }
};

// EMPInt64Nodes are long values
class EMPInt64Node : public EMPNode
{
public:
    long value;
    std::vector<unsigned char> encode();

    EMPInt64Node(long val)
    {
        value = val;
    }
};

// EMPInt32Nodes are int values
class EMPInt32Node : public EMPNode
{
public:
    int value;
    std::vector<unsigned char> encode();

    EMPInt32Node(int val)
    {
        value = val;
    }
};

// EMPInt16Nodes are short values
class EMPInt16Node : public EMPNode
{
public:
    short value;
    std::vector<unsigned char> encode();

    EMPInt16Node(short val)
    {
        value = val;
    }
};

// EMPInt8Nodes are char values
//
// C++ considers chars signed bytes, so this is the best way to encode a 8 bit
// integer
class EMPInt8Node : public EMPNode
{
public:
    char value;
    std::vector<unsigned char> encode();

    EMPInt8Node(char val)
    {
        value = val;
    }
};

// EMPFloatNodes are float values
class EMPFloatNode : public EMPNode
{
public:
    float value;
    std::vector<unsigned char> encode();

    EMPFloatNode(float val)
    {
        value = val;
    }
};

// EMPDoubleNodes are double values
class EMPDoubleNode : public EMPNode
{
public:
    double value;
    std::vector<unsigned char> encode();

    EMPDoubleNode(double val)
    {
        value = val;
    }
};

// EMPArrayNodes are an std::vector<EMPNode>
//
// This allows for any value to be passed into the array, allowing for the data
// structure that is used for JSON, NBT, and EMP files
class EMPArrayNode : public EMPNode
{
public:
    std::vector<EMPNode> value;
    std::vector<unsigned char> encode();

    EMPArrayNode(std::vector<EMPNode> val)
    {
        value = val;
    }
};

// EMPObjectNodes are an std::map<std::string, EMPNode>
//
// This and EMPArrayNodes are the main reason the EMPNode system is set up in
// the first place, as this allows for a seamless way of letting arrays
// contain arrays and objects contain objects
class EMPObjectNode : public EMPNode
{
public:
    std::map<std::string, EMPNode> value;
    std::vector<unsigned char> encode();

    EMPObjectNode(std::map<std::string, EMPNode> val)
    {
        value = val;
    }
};