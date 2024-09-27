#ifndef ENCODE_HEADER_H
#define ENCODE_HEADER_H

#include <iostream>
#include <vector>
#include "structure.h"
#include <tuple>

std::vector<char> encode(EMPData data);

std::tuple<std::vector<char>, int> int_to_vec(long numb, char size);

#endif