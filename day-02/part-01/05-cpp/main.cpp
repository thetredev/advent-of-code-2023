#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <sstream>
#include <vector>


int lex(std::ifstream &input) {
    int result = 0;
    std::string line;

    const int MAX_RED = 12;
    const int MAX_GREEN = 13;
    const int MAX_BLUE = 14;

    int game_id = 0;

    while (std::getline(input, line)) {
        game_id++;

        size_t data_start = line.find(':') + 2;
        line = line.erase(0, data_start);

        bool possible = true;

        while (line.front() >= ' ') {
            size_t digit_index = 0;

            while (std::isdigit(line.at(digit_index))) {
                digit_index++;
            }

            int value = std::stoi(line.substr(0, digit_index));

            line.erase(0, digit_index + 1);
            const char color_code = line.front();

            if (color_code == 'r' && value > MAX_RED) {
                possible = false;
                break;
            }

            if (color_code == 'g' && value > MAX_GREEN) {
                possible = false;
                break;
            }

            if (color_code == 'b' && value > MAX_BLUE) {
                possible = false;
                break;
            }

            while (!std::isdigit(line.front())) {
                if (line.front() < ' ') {
                    break;
                }

                line.erase(0, 1);
            }
        }

        if (possible) {
            result += game_id;
        }
    }

    return result;
}


int main(int argc, char** argv) {
    std::ifstream input(argv[1]);
    std::cout << lex(input) << std::endl;
    input.close();

    return 0;
}
