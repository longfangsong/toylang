#include "node.h"
#include <stdio.h>
#include <stdlib.h>

static void generate_code(ASTNode *node) {
  fprintf(stderr, "generate_code not defined for this!");
  exit(1);
}

void init_ast_node(ASTNode *node, NodeType type) {
  node->type = type;
  node->generate_code = generate_code;
}