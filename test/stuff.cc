#include <stdio.h>

extern int back_to_main;

void test(int i) {
  printf("from main %d, modified %d\n", back_to_main, i);
}
