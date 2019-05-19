#ifndef TOYLANG_COMPOUND_H
#define TOYLANG_COMPOUND_H

#include "../statement.h"
#include <stdlib.h>

typedef struct CompoundStatement {
    Statement base;
    size_t child_count;
    Statement **children;
    size_t namespace_id;
} CompoundStatement;

CompoundStatement *create_compound_statement();

void add_statement(CompoundStatement *compoundStatement, Statement *statement);

#endif //TOYLANG_COMPOUND_H
