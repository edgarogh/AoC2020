#include <stdio.h>
#include <stdbool.h>
#include <string.h>

int main() {
    FILE* input = fopen("input.txt", "r");

    char password[100];
    int valid = 0;

    while (!feof(input)) {
        int min, max;
        char c;
        fscanf(input, "%d-%d %c: %s\n", &min, &max, &c, password);

        int password_len = strlen(password);

        int count = 0;
        for (int i = 0; i < password_len; i++) {
            if (c == password[i]) {
                count++;
            }
        }

        if (min <= count && count <= max) {
            valid++;
        }
    }

    printf("valid passwords: %d\n", valid);

    fclose(input);
    return 0;
}
