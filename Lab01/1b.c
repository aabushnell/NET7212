#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  printf("Not Segfaulting...\n");

  char s1[16];
  char s2[16];

  strncpy(s1, "string 01", 16);
  strncpy(s2, "string 02", 16);

  for (int i = 0; i < 24; i++) {
    s2[i] = 'A';
  }

  printf("s1 = %s\n", s1);
  printf("s2 = %s\n", s2);

  return EXIT_SUCCESS;
}
