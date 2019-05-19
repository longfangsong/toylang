#ifndef TOYLANG_SYMBOLTYPE_H
#define TOYLANG_SYMBOLTYPE_H

typedef enum {
    Bool,
    Char,
    Int,
    Double,
    String,
    Array
} SymbolType;

const char *type_string(SymbolType type);

#endif //TOYLANG_SYMBOLTYPE_H
