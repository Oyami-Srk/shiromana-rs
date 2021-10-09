#include "plugin.h"
#include <stdio.h>

const char *name() {
    return "test_plugin_v1";
}

const char *trigger() {
    return "on_load";
}

unsigned int on_load(struct Library *lib) {
    printf("Test on_load\n");
    return 0;
}

unsigned int on_unload() {
    printf("Test on_unload\n");
    return 0;
}

unsigned int on_trigger() {
    printf("trigger.\n");
    return 0;
}