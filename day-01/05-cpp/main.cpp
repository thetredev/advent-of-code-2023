#include <fstream>
#include <iostream>
#include <string>
#include <vector>


int lex(std::ifstream &input) {
    int result = 0;
    std::string line;

    while (input >> line) {
        std::vector<char> digits;

        for (int i = 0; i < line.length(); i++) {
            char c = line[i];

            if (std::isdigit(c)) {
                digits.push_back(c);
            }
        }

        char resultChars[3];
        resultChars[0] = digits.front();
        resultChars[1] = digits.back();
        resultChars[2] = '\0';

        result += std::stoi(resultChars);
    }

    return result;
}


int main(int argc, char** argv) {
    std::ifstream input(argv[1]);
    std::cout << lex(input) << std::endl;
    input.close();

    return 0;
}
