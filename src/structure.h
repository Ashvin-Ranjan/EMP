#ifndef STRUCT_HEADER_H
#define STRUCT_HEADER_H

#include <iostream>
#include <vector>

enum EMPDataType {
    INT_8,
    INT_16,
    INT_32,
    INT_64,
    NONE,
    BOOL,
    ARRAY,
    FLOAT,
    STRING,
};

struct EMPData
{
    EMPDataType type;
    void* data;
};

#endif