#include <iostream>
#include <fstream>
#include "structure.h"
#include "encode.h"
#include "to_string.h"
#include "from_string.h"

int main() {
    // float f_val = 56334.34F;
    // EMPData byte = {FLOAT, &f_val};
    // std::cout << "test\n";
    // std::vector<char> data = encode(byte);
    // std::printf("%s\n", to_string(byte).c_str());
    // std::ofstream outfile("./data.emp", std::ios::out | std::ios::binary); 
    // outfile.write(&data[0], data.size());
    std::vector<std::string> val = tokenize("null");
    for (int i = 0; i < val.size(); i++) {
        std::cout << '"' << val.at(i) << '"' << ", ";
    }
    std::cout << std::endl;

    return 0;
}