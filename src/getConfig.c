#include "string.util.c"
struct Config
{
    char *name;
    char *value;
    struct Config *next;
};

struct Config *initConfig(char *configFile)
{
    FILE *file = fopen(configFile, "r");
    if (!file)
    {
        printf("Error: File not found\n");
        return NULL;
    }
    char line[256];
    struct Config *head = NULL;
    while (fgets(line, sizeof(line), file))
    {
        if (line[0] == '#')
        {
            continue;
        }
        if (line[0] == '\n')
        {
            continue;
        }
        // example config line is in="print"
        char *name = sliceChar(line, 0, indexOf(line, '='));
        char *value = sliceChar(line, indexOf(line, '=') + 1, strlen(line));
        value = replaceWord(value, "\"", "");
        value = trimString(value);
        struct Config *newConfig = (struct Config *)malloc(sizeof(struct Config));
        newConfig->name = name;
        newConfig->value = value;
        newConfig->next = head;
        head = newConfig;
    }
    return head;
}

// find config start with name
struct Config *findConfig(char *command, struct Config *configs)
{
    struct Config *config = configs;
    while (config)
    {
        if (startsWith(config->name, command))
        {
            return config;
        }
        config = config->next;
    }
    return NULL;
}

struct Config *findConfigWithName(char *command, struct Config *configs)
{
    struct Config *config = configs;
    while (config)
    {
        char **command = str_split(command," ");
        
        if (CompareString(config->name, command))
        {
            return config;
        }
        config = config->next;
    }
    return NULL;
}
