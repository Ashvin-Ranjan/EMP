#include "to_string.h"
#include "constants.h"

std::string to_string(EMPData data) {
    std::string out;

    switch (data.type) {
        case NONE:
            out = EMPConst::EMP_NULL;
            break;
        case INT_8:
        {
            char data_val = *(char*)data.data;
            out = std::to_string(data_val) + EMPConst::EMP_BYTE;
            break;
        }
        case INT_16:
        {
            short data_val = *(short*)data.data;
            out = std::to_string(data_val) + EMPConst::EMP_SHORT;
            break;
        }
        case BOOL:
        {
            bool data_val = *(bool*)data.data;
            out = data_val ? EMPConst::EMP_TRUE : EMPConst::EMP_FALSE;
            break;
        }
        case ARRAY:
        {
            std::vector<EMPData> data_val = *(std::vector<EMPData>*)data.data;
            out = EMPConst::EMP_OPEN_BRACE;
            for (EMPData d : data_val) {
                out += to_string(d) + ',';
            }
            out.pop_back();
            out += EMPConst::EMP_CLOSE_BRACE;
        }
    }

    return out;
}