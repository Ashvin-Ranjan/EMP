#include <iostream>
#include <fstream>
#include <vector>

#include "types/types.hpp"

using namespace std;

int main()
{
    ofstream fout;
    fout.open("encode.emp", ios::binary | ios::out);

    vector<unsigned char> data = EMPInt64Node(2LL).encode();
    fout.write((const char *)data.data(), data.size());

    fout.close();

    return 0;
}