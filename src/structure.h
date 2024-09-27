#ifndef STRUCT_HEADER_H
#define STRUCT_HEADER_H

#include <iostream>
#include <vector>

enum EMPDataType {
    INT_8,
    INT_16,
    NONE,
    BOOL,
    ARRAY,
};

struct EMPData
{
    EMPDataType type;
    void* data;
};

#endif