#include <stdio.h>
#include "string.util.c"
#include "getConfig.c"

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
        struct Config *configs = initConfig("./keyword.config");
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
                struct Config *config = findConfig(line, configs);
                if (CompareString(config->value, "print"))
                {
                    // print the line
                    // printf("%s\n", config->name);
                    char *PrintStr = replaceWord(line, concat(config->name, ":"), "");
                    PrintStr = trimString(PrintStr);
                    PrintStr = replaceWord(PrintStr, "\"", "");
                    printf("%s\n", PrintStr);
                }
            }
        }
    }
    return EXIT_SUCCESS;
}
