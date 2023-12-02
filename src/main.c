#include <stdio.h>
typedef void logging_function(const char *ptr, unsigned int size);
extern void run_callback(logging_function *log);

void log_stuff(const char *ptr, unsigned int size) {
    printf("%.*s\n", (int)size, (char *)ptr);
}

int main(int argc, char **argv) {
    run_callback(&log_stuff);
    return 0;
}