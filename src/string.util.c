#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <assert.h>

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

bool endWith(const char *string, const char *suffix)
{
    if (!string || !suffix)
        return false;
    size_t lenStr = strlen(string);
    size_t lenSuffix = strlen(suffix);
    if (lenSuffix > lenStr)
        return false;
    return strncmp(string + lenStr - lenSuffix, suffix, lenSuffix) == 0;
}

bool isNumber(char *str)
{
    if (strlen(str) == 0)
    {
        return false;
    }
    int j = 0;
    while (j < strlen(str))
    {
        if (!(str[j] >= '0' && str[j] <= '9'))
        {
            return false;
        }
        j++;
    }
    return true;
}

char **strSplit(char *a_str, const char a_delim)
{
    char **result = 0;
    size_t count = 0;
    char *tmp = a_str;
    char *last_comma = 0;
    char delim[2];
    delim[0] = a_delim;
    delim[1] = 0;

    /* Count how many elements will be extracted. */
    while (*tmp)
    {
        if (a_delim == *tmp)
        {
            count++;
            last_comma = tmp;
        }
        tmp++;
    }

    /* Add space for trailing token. */
    count += last_comma < (a_str + strlen(a_str) - 1);

    /* Add space for terminating null string so caller
       knows where the list of returned strings ends. */
    count++;

    result = malloc(sizeof(char *) * count);

    if (result)
    {
        size_t idx = 0;
        char *token = strtok(a_str, delim);

        while (token)
        {
            assert(idx < count);
            *(result + idx++) = strdup(token);
            token = strtok(0, delim);
        }
        assert(idx == count - 1);
        *(result + idx) = 0;
    }

    return result;
}