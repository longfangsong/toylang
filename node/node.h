#ifndef __TOYLANG__NODE__NODE_H__
#define __TOYLANG__NODE__NODE_H__

typedef enum {
  Const,
  Variable,
  BinaryOp,

  Declare = 128,
  Assign,
  If,
  While,
  StatementGroup = 255
} NodeType;

typedef struct AST_Node {
  NodeType type;
  void (*generate_code)(struct AST_Node *node);
} ASTNode;

void init_ast_node(ASTNode *node, NodeType type);

#endif