#ifndef TOYLANG_NODE_H
#define TOYLANG_NODE_H

#include <stdlib.h>

typedef struct ASTNode {
#ifdef DEBUG

    void (*print_ast_node)(struct ASTNode *node, size_t layer);

#endif

    void (*free_node)(struct ASTNode *node);
} ASTNode;

#endif //TOYLANG_NODE_H
