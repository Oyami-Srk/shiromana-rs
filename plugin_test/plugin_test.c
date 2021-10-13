#include "plugin.h"
#include <stdio.h>

const char *name() { return "test_plugin_v1"; }

const char *trigger() { return "media_add,media_remove"; }

unsigned int on_load(struct Library *lib) {
  printf("Test on_load\n");
  return 0;
}

unsigned int on_unload(struct Library *lib) {
  printf("Test on_unload\n");
  return 0;
}

unsigned int on_trigger(struct Library *lib, struct Media *media,
                        const char *trigger_type) {
  printf("This plugin triggerred.\n");
  return 0;
}