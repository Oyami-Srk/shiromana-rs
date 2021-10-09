#ifndef __SHIROMANA_RS_PLUGIN_H__
#define __SHIROMANA_RS_PLUGIN_H__

#include <stdlib.h>

struct Library;

const char *name();
const char *trigger();
unsigned int on_load(struct Library *lib);
unsigned int on_unload();
unsigned int on_trigger();

#endif // __SHIROMANA_RS_PLUGIN_H__