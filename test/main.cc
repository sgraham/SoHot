#include <windows.h>

int back_to_main = 1234;

void sohot_process(void) {
  static volatile bool hack = false;

  if (hack) {
    LoadLibrary("test.hot_1.exe");
  }
}

void test(int i);

int main() {
  int i = 0;
  for (;;) {
    Sleep(500);
    test(i++);
    sohot_process();
  }
}

