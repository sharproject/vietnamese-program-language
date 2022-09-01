// #include "string.util.c"
struct Map
{
    char *key;
    char *value;
    int *IntValue;
    struct Map *next;
};

struct Map *head = NULL;

void newVariable(char *name, char *value)
{
    struct Map *newConfig = (struct Map *)malloc(sizeof(struct Map));
    newConfig->key = name;
    newConfig->value = value;
    newConfig->next = head;
    newConfig->IntValue = isNumber(value) ? (int *)value : NULL;
    head = newConfig;
}

char *getVariableValue(char *name)
{
    struct Map *item = head;
    while (item)
    {
        if (CompareString(item->key, name))
        {
            return item->value;
        }
        item = item->next;
    }
    return NULL;
}

struct Map *getVariable(char *name)
{
    struct Map *item = head;
    while (item)
    {
        if (CompareString(item->key, name))
        {
            return item;
        }
        item = item->next;
    }
    return NULL;
}
