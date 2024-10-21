#include "constants.h"
#include "from_string.h"

bool in_char_list(const char* list, char input, int length) {
    for (int i = 0; i < length; i++) {
        if (list[i] == input) {
            return true;
        }
    }
    return false;
}

bool compare_strings(char* check, char* input) {
    while (*check != '\0' && *input != '\0') {
        if (*check++ != *input++) {
            return false;
        }
    }
    return *check == '\0';
}

std::vector<std::string> tokenize(char* input) {
    std::vector<std::string> out;
    while (*input != '\0') {
        // Automatically seperate out all "control characters"
        if (in_char_list(EMPConst::EMP_CONTROL, *input, 6)) {
            out.push_back(std::string() + *input);
        }
        // If there is a quote then work through the string
        else if (*input == EMPConst::EMP_QUOTE) {
            std::string str = std::string() + '"';
            input++;
            while (*input != '"') {
                // If the input ends halfway through the string then throw an error
                if (*input == '\0') {
                    throw std::invalid_argument("Unable to parse string");
                }
                // Handle different types of escape codes
                if (*input == '\\') {
                    input++;
                    switch (*input) {
                        case '\\':
                            str += '\\';
                        break;
                        case 'n':
                            str += '\n';
                        break;
                        case 'r':
                            str += '\r';
                        break;
                        case 't':
                            str += '\n';
                        break;
                        case '"':
                            str += '"';
                        break;
                        default:
                            throw std::invalid_argument("Unable to parse string");
                        break;
                    }
                } else {
                    str += *input;
                }
                input++;
            }
            // Push a final quote onto the string
            str += '"';
            out.push_back(str);
        }
        // If there is a number or a decimal then start to parse a number
        // Note: that this will parse "." as a valid number, the token handler will
        // have to catch this
        else if ((*input > 47 && *input < 58) || *input == EMPConst::EMP_PERIOD) {
            std::string str = std::string() + *input++;
            while ((*input > 47 && *input < 58) || *input == EMPConst::EMP_PERIOD) {
                str += *input++;
            }
            // Check for an idenfitier at the end of the string
            if (in_char_list(EMPConst::EMP_NUMB_SIGN, *input, 5)) {
                str += *input;
            } else {
                // If there isn't one then decrease the input pointer for the input++ at the end of the loop
                input--;
            }
            out.push_back(str);
        }
        else if (compare_strings((char*)EMPConst::EMP_NULL, input)) {
            out.push_back(std::string(EMPConst::EMP_NULL));
            // Add 3 to input so that after the input++ it will be 4
            input += 3;
        }
        else if (compare_strings((char*)EMPConst::EMP_TRUE, input)) {
            out.push_back(std::string(EMPConst::EMP_TRUE));
            // Add 3 to input so that after the input++ it will be 4
            input += 3;
        }
        else if (compare_strings((char*)EMPConst::EMP_FALSE, input)) {
            out.push_back(std::string(EMPConst::EMP_FALSE));
            // Add 4 to input so that after the input++ it will be 5
            input += 4;
        }
        // Ignore whitespace
        else if (in_char_list(EMPConst::EMP_WHITESPACE, (*input), 4)) {}
        else {
            // If there was nothing able to be matched then throw an error
            throw std::invalid_argument("Unable to parse string");
        }
        input++;
    }
    return out;
}