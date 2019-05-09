#include "binaryOperator.h"
#include "../../../../y.tab.h"
#include <stdio.h>
struct {
  size_t operator;
  const char *ir_code;
} operator_ir_map[] = {{'+', "add"},  {'-', "sub"},     {'*', "mul"},
                       {'/', "sdiv"}, {'<', "slt"},     {'>', "sge"},
                       {EQUAL, "eq"}, {NONEQUAL, "neq"}};

const char *get_ir(size_t operator) {
  for (size_t i = 0; i < 8; ++i) {
    if (operator_ir_map[i].operator== operator) {
      return operator_ir_map[i].ir_code;
    }
  }
  return NULL;
}

static void generate_code(BinaryOpNode *node) {
  switch (node->operation) {
  case '+':
  case '-':
  case '*':
  case '/':
    ((ASTNode *)(node->lhs))->generate_code((ASTNode *)(node->lhs));
    ((ASTNode *)(node->rhs))->generate_code((ASTNode *)(node->rhs));
    printf("%s = %s i32 ",
           ((ExpressionNode *)node)->get_value_string((ExpressionNode *)node),
           get_ir(node->operation));
    printf("%s, %s\n", node->lhs->get_value_string(node->lhs),
           node->rhs->get_value_string(node->rhs));
    break;
  case '<':
  case '>':
  case EQUAL:
  case NONEQUAL:
    ((ASTNode *)(node->lhs))->generate_code((ASTNode *)(node->lhs));
    ((ASTNode *)(node->rhs))->generate_code((ASTNode *)(node->rhs));
    printf("%s = icmp %s i32 ",
           ((ExpressionNode *)node)->get_value_string((ExpressionNode *)node),
           get_ir(node->operation));
    printf("%s, %s\n", node->lhs->get_value_string(node->lhs),
           node->rhs->get_value_string(node->rhs));
    break;
  default:
    break;
  }
}

BinaryOpNode *create_binary_op_node(size_t operation, ExpressionNode *lhs,
                                    ExpressionNode *rhs) {
  BinaryOpNode *result = (BinaryOpNode *)malloc(sizeof(BinaryOpNode));
  init_register_node((TempRegisterNode *)(result), BinaryOp);
  ((ASTNode *)result)->generate_code = (void (*)(ASTNode *))(generate_code);
  result->operation = operation;
  result->lhs = lhs;
  result->rhs = rhs;
  return result;
}