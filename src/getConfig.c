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
    char line[2047];
    int checked = 0;
    struct Config *head = NULL;
    while(fgets(line, 2047, file) != NULL) {
        char *key;
        char *name;
        char *value;
        int i, j;

        if (line[0] == '#')
        {
            continue;
        }
        if (line[0] == '\n')
        {
            continue;
        }
        key = sliceChar(line, indexOf(line, '<') + 1, strlen(line));
        key = sliceChar(key, 0 + indexOf(key, '!') + 1, strlen(key));
        key = replaceWord(key, "DOCTYPE Note [", "");
        key = sliceChar(key, 0 + indexOf(key, '>') + 1, strlen(key));
        key = sliceChar(key, 0, indexOf(key, '<'));

        struct Config *newConfig = (struct Config *)malloc(sizeof(struct Config));

        if (startsWith(key, "\n")) continue;
        if (checked == 0) {
            name = key;
            checked = 1;
        } else {
            checked = 0;
            value = key;
            value = replaceWord(value, "\"", "");
            value = trimString(value);
        }
        newConfig->name = name;
        newConfig->value = value;
        newConfig->next = head;
        head = newConfig;
    };
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
