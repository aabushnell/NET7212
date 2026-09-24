#include <stdio.h>
#include <stdlib.h>

void get_data(char *data, int pos) {
  printf("[public] data[%d] = %c\n", pos, data[pos]);
}

int main(int argc, char **argv) {
  printf("Not Segfaulting...\n");

  int pos = atoi(argv[1]);

  printf("pos = %d\n", pos);

  char passwd[16] = {'1', '2', '3', '4', '\0'};
  char data[8] = {'D', 'A', 'T', 'A', '\0'};

  printf("[private] passwd = %s\n", passwd);
  printf("[private] data = %s\n", data);

  for (int i = 0; i < 4; i++) {
    get_data(data, pos + i);
  }

  return EXIT_SUCCESS;
}
