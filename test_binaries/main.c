#include <stdio.h>

void foo() {}

int g_bla = 5;
void* g_foo_ptr = foo;

int main() {
    printf("%x, %p\n", g_bla, g_foo_ptr);
    return 0;
}
