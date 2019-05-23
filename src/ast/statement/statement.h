#ifndef TOYLANG_STATEMENT_H
#define TOYLANG_STATEMENT_H

#include "../node.h"

typedef struct Statement {
    ASTNode base;

    void (*generate_code)(struct Statement *node);
} Statement;

#endif //TOYLANG_STATEMENT_H
