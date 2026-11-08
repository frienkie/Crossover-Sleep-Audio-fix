// Minimal native entry point; the engine owns all routing and cleanup.
#include <stdbool.h>
#include <string.h>

extern int sleep_audio_main(bool list_only);

int main(int argc, char **argv) {
    if (argc > 2 || (argc == 2 && strcmp(argv[1], "--list"))) return 2;
    return sleep_audio_main(argc == 2);
}
