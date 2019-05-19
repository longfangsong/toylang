#include "symbolType.h"
#include <stdlib.h>

const char *type_string(SymbolType type) {
    switch (type) {
        case Bool:
            return "i1";
        case Int:
            return "i32";
        case Double:
            return "double";
        default:
            return NULL;
    }
}