# GNU Makefile
# plugin

CC=gcc
CFLAGS=-Wall -Werror
OBJ=plugin.o
PLUGIN=plugin.so

$(PLUGIN): $(OBJ)
	$(CC) -shared -o $@ $<

$(OBJ): plugin_test.c
	$(CC) -c -fpic -o $@ $< $(CFLAGS)