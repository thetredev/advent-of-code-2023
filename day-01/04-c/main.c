#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int lex(FILE *fp) {
    int result = 0;
    int digits = 0;
    char lastChar = '\0';
    char buffer[1];
    char chars[50];

    fseek(fp, 0L, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0L, SEEK_SET);

    for (long i = 0; i < size; i++) {
        fread(buffer, sizeof(char), 1, fp);
        lastChar = buffer[0];

        if (lastChar == '\n' || i == size - 1) {
            char resultChars[3];
            resultChars[0] = chars[0];
            resultChars[1] = chars[digits - 1];
            resultChars[2] = '\0';

            result += atoi(resultChars);

            digits = 0;
            lastChar = '\0';
        } else if (isdigit(lastChar)) {
            chars[digits] = lastChar;
            digits++;
        }
    }

    return result;
}


int main(int argc, char** argv) {
    FILE *fp = fopen(argv[1], "r");
    printf("%d\n", lex(fp));
    fclose(fp);

    return 0;
}
