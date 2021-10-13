#ifndef __SHIROMANA_RS_PLUGIN_H__
#define __SHIROMANA_RS_PLUGIN_H__

#include <stdlib.h>

struct Library;
struct Media;

const char *name();
const char *trigger();
unsigned int on_load(struct Library *lib);
unsigned int on_unload(struct Library *lib);
unsigned int on_trigger(struct Library *lib, struct Media *media,
                        const char *trigger_type);

#endif // __SHIROMANA_RS_PLUGIN_H__