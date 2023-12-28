#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int lex(FILE *fp) {
    int result = 0;

    char* cursor = NULL;
    size_t line_buffer;
    size_t chars_read;

    const size_t MAX_RED = 12;
    const size_t MAX_GREEN = 13;
    const size_t MAX_BLUE = 14;

    size_t game_id = 0;

    while ((chars_read = getline(&cursor, &line_buffer, fp)) != -1) {
        game_id++;

        while (*cursor++ != ':');
        cursor++;

        int possible = 1;

        while (*cursor >= ' ') {
            char color_value_str[5];
            size_t index = 0;

            while (isdigit(*cursor)) {
                color_value_str[index] = *cursor;
                cursor++;
                index++;
            }

            color_value_str[index] = '\0';
            int color_value = atoi(color_value_str);

            // skip space
            cursor++;

            // check color code
            if (*cursor == 'r') {
                if (color_value > MAX_RED) {
                    possible = 0;
                    break;
                }

                cursor += 3;
            } else if (*cursor == 'g') {
                if (color_value > MAX_GREEN) {
                    possible = 0;
                    break;
                }

                cursor += 5;
            } else if (*cursor == 'b') {
                if (color_value > MAX_BLUE) {
                    possible = 0;
                    break;
                }

                cursor += 4;
            }

            cursor++;

            if (*cursor < ' ') {
                break;
            }

            while (!isdigit(*cursor)) {
                cursor++;
            }
        }

        if (possible == 1) {
            result += game_id;
        }

        line_buffer = 0;
        chars_read = 0;
    }

    return result;
}


int main(int argc, char** argv) {
    FILE *fp = fopen(argv[1], "r");
    printf("%d\n", lex(fp));
    fclose(fp);

    return 0;
}
