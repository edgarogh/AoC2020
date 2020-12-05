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

        #ifdef PART_ONE
        int count = 0;
        for (int i = 0; password[i] != 0; i++) {
            if (c == password[i]) {
                count++;
            }
        }

        if (min <= count && count <= max) {
            valid++;
        }
        #endif

        #ifdef PART_TWO
        bool min_eq = password[min - 1] == c;
        bool max_eq = password[max - 1] == c;
        if (min_eq != max_eq) {
            valid++;
        }
        #endif
    }

    printf("valid passwords: %d\n", valid);

    fclose(input);
    return 0;
}
