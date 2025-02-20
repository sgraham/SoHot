#include <stdio.h>

extern int back_to_main;

void test(int i) {
  printf("from main %d, in test %d\n", back_to_main, i);
}
