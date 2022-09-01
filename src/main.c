#include <stdio.h>
#include "getConfig.c"
#include "process.c"

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
        struct Config *configs = initConfig("./config/keyword.xml");
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
                process(line, configs);
            }
        }
    }
    return EXIT_SUCCESS;
}
