#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>


bool startsWith(const char *pre, const char *str)
{
    size_t lenPrefix = strlen(pre),
           lenString = strlen(str);
    return lenString < lenPrefix ? false : memcmp(pre, str, lenPrefix) == 0;
}
char* replaceWord(const char* s, const char* oldW,
                const char* newW);

int main(int argc, char *argv[])
{
    // if !argv[1]
    if (!argv[1])
    {
        printf("Usage: %s <filename> to run the vipl file\n", argv[0]);
        return 1;
    }
    else
    {
        // open file
        FILE *file = fopen(argv[1], "r");
        // if file == NULL
        if (!file)
        {
            printf("Error: File not found\n");
            return 1;
        }
        // read each line of the file
        char line[256];
        while (fgets(line, sizeof(line), file))
        {
            if (line[0] == '#')
            {
                continue;
            }
            else
            {
                // if line start with "in:"
                if (startsWith("in_ra_màn_hình:", line))
                {
                    // print the line
                    char *PrintStr = replaceWord(line,"in_ra_màn_hình:", "");
                    PrintStr = replaceWord(PrintStr,"\"", "");
                    printf("%s\n", PrintStr);
                }
            }
        }
    }
    return 0;
}

// You must free the result if result is non-NULL.
char* replaceWord(const char* s, const char* oldW,
                const char* newW)
{
    char* result;
    int i, cnt = 0;
    int newWlen = strlen(newW);
    int oldWlen = strlen(oldW);
 
    // Counting the number of times old word
    // occur in the string
    for (i = 0; s[i] != '\0'; i++) {
        if (strstr(&s[i], oldW) == &s[i]) {
            cnt++;
 
            // Jumping to index after the old word.
            i += oldWlen - 1;
        }
    }
 
    // Making new string of enough length
    result = (char*)malloc(i + cnt * (newWlen - oldWlen) + 1);
 
    i = 0;
    while (*s) {
        // compare the substring with the result
        if (strstr(s, oldW) == s) {
            strcpy(&result[i], newW);
            i += newWlen;
            s += oldWlen;
        }
        else
            result[i++] = *s++;
    }
 
    result[i] = '\0';
    return result;
}