#include "symbolTable.h"
#include <string.h>

SymbolTableStack symbol_table_stack = {0, NULL};

size_t next_frame_id = 0;

void push_frame() {
    SymbolTableFrame *new_frame = malloc(sizeof(SymbolTableFrame));
    new_frame->symbols = NULL;
    new_frame->length = 0;
    new_frame->namespace_id = next_frame_id++;
    symbol_table_stack.frames = realloc(symbol_table_stack.frames,
                                        sizeof(SymbolTableFrame *) * (symbol_table_stack.length + 1));
    symbol_table_stack.frames[symbol_table_stack.length] = new_frame;
    ++symbol_table_stack.length;
}

void pop_frame() {
    free(symbol_table_stack.frames[symbol_table_stack.length - 1]);
    --symbol_table_stack.length;
}

void add_symbol(Symbol *symbol) {
    SymbolTableFrame *frame = symbol_table_stack.frames[symbol_table_stack.length - 1];
    symbol->namespace_id = frame->namespace_id;
    frame->symbols = realloc(frame->symbols, (frame->length + 1) * sizeof(Symbol *));
    frame->symbols[frame->length] = symbol;
    ++frame->length;
}

Symbol *get_symbol(char *name) {
    for (size_t i = symbol_table_stack.length; i > 0; --i) {
        SymbolTableFrame *frame = symbol_table_stack.frames[i - 1];
        for (size_t j = 0; j < frame->length; ++j) {
            if (strcmp(name, frame->symbols[j]->name) == 0) {
                return frame->symbols[j];
            }
        }
    }
    return NULL;
}