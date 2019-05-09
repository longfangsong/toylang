#ifndef __TOYLANG__NODE__EXPRESSION__CONSTANT__CONSTANT_H__
#define __TOYLANG__NODE__EXPRESSION__CONSTANT__CONSTANT_H__
#include "../expression.h"
typedef struct {
  ExpressionNode base;
  int value;
} ConstantNode;

ConstantNode *create_constant_node(int value);

#endif
