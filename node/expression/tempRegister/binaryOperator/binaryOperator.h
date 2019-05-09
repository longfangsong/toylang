#ifndef __TOYLANG__NODE__EXPRESSION__TOREGISTER__BINOP__BINOP_H__
#define __TOYLANG__NODE__EXPRESSION__TOREGISTER__BINOP__BINOP_H__
#include "../tempRegister.h"

typedef struct {
  TempRegisterNode base;
  size_t operation;
  ExpressionNode *lhs;
  ExpressionNode *rhs;
} BinaryOpNode;

BinaryOpNode *create_binary_op_node(size_t operation, ExpressionNode *lhs,
                                    ExpressionNode *rhs);

#endif
