#include "string.util.c"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  FILE* file;
  
  int checked = 0;
  char s[2048];

  file = fopen("./config/keyword.xml", "r");

  if (file == NULL) return printf("File error, can't read! n");
  while(fgets(s, 2047, file) != NULL) {
    char *key;
    
    int i, j;

    key = sliceChar(s, indexOf(s, '<') + 1, strlen(s));
    key = sliceChar(key, 0 + indexOf(key, '!') + 1, strlen(key));
    key = replaceWord(key, "DOCTYPE Note [", "");
    key = sliceChar(key, 0 + indexOf(key, '>') + 1, strlen(key));
    key = sliceChar(key, 0, indexOf(key, '<'));
    
    if (startsWith(key, "\n")) continue;
    if (checked == 0) {
      printf("%s=", key);
      checked = 1;
    } else {
      printf("%s\n", key);
      checked = 0;
    }
    
  };
  
  fclose(file);
  return 0;
}
