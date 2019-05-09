#ifndef __TOYLANG__NODE__STATEMENT__DECLARE__DECLARE_H__
#define __TOYLANG__NODE__STATEMENT__DECLARE__DECLARE_H__
#include "../../expression/tempRegister/variable/variable.h"
#include "../statement.h"

typedef struct Declare_Node {
  StatementNode base;
  VariableNode *variable;
} DeclareNode;

DeclareNode *create_declare_node(VariableNode *variable);

#endif
