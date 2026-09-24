#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

char global_buf[1] = {0};

void fault1(bool enable) {
  // heap
  if (!enable) {
    return;
  }
  printf("~~ Fault 1 ~~\n");

  char *buf = malloc(1 * sizeof(*buf));

  buf[10000000000] = 1;

  return;
}

void fault2(bool enable) {
  // data
  if (!enable) {
    return;
  }
  printf("~~ Fault 2 ~~\n");

  *((char *)global_buf + 100000) = 1;

  return;
}

void fault3(bool enable) {
  // stack
  if (!enable) {
    return;
  }
  printf("~~ Fault 3 ~~\n");

  char buf[1] = {0};

  *((char *)buf + 100000) = 1;

  printf("%c", buf[0]);

  return;
}

int main(int argc, char **argv) {
  printf("Segfaulting...\n");

  fault1(false);

  fault2(true);

  fault3(true);

  return EXIT_SUCCESS;
}
