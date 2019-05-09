#ifndef __TOYLANG__NODE__EXPRESSION__TOREGISTER__TOTEGISTER_H__
#define __TOYLANG__NODE__EXPRESSION__TOREGISTER__TOTEGISTER_H__
#include "../expression.h"
#include <stdlib.h>

typedef struct {
  ExpressionNode base;
  size_t register_id;
} TempRegisterNode;

void init_register_node(TempRegisterNode *node, NodeType type);

#endif
