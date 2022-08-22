#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
bool startsWith(const char *pre, const char *str)
{
    size_t lenPrefix = strlen(pre),
           lenString = strlen(str);
    return lenString < lenPrefix ? false : memcmp(pre, str, lenPrefix) == 0;
}

char *replaceWord(const char *s, const char *oldW, const char *newW)
{
    char *result = malloc(strlen(s) + 1);
    char *p = result;
    while (*s)
    {
        if (startsWith(oldW, s))
        {
            strcpy(p, newW);
            p += strlen(newW);
            s += strlen(oldW);
        }
        else
        {
            *p++ = *s++;
        }
    }
    *p = 0;
    return result;
}

char *trimString(const char *s)
{
    char *result = malloc(strlen(s) + 1);
    char *p = result;
    while (*s && isspace(*s))
    {
        s++;
    }
    while (*s)
    {
        *p++ = *s++;
    }
    while (p > result && isspace(*(p - 1)))
    {
        p--;
    }
    *p = 0;
    return result;
}

int indexOf(const char *s, char c)
{
    int i = 0;
    while (*s)
    {
        if (*s == c)
        {
            return i;
        }
        i++;
        s++;
    }
    return -1;
}

char *sliceChar(const char *s, int start, int end)
{
    char *result = malloc(end - start + 1);
    char *p = result;
    while (start < end)
    {
        *p++ = s[start++];
    }
    *p = 0;
    return result;
}

bool CompareString(const char *s1, const char *s2)
{
    if (strlen(s1) != strlen(s2))
    {
        return false;
    }
    for (int i = 0; i < strlen(s1); i++)
    {
        if (s1[i] != s2[i])
        {
            return false;
        }
    }
    return true;
}
char *concat(const char *s1, const char *s2)
{
    char *result = malloc(strlen(s1) + strlen(s2) + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}