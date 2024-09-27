#include <iostream>
#include <fstream>
#include "structure.h"
#include "encode.h"
#include "to_string.h"

int main() {
    EMPData byte = {NONE, NULL};
    std::cout << "test\n";
    std::vector<char> data = encode(byte);
    std::printf("%s\n", to_string(byte).c_str());
    std::ofstream outfile("./data.emp", std::ios::out | std::ios::binary); 
    outfile.write(&data[0], data.size());
    return 0;
}