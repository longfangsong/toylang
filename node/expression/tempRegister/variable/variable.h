#ifndef __TOYLANG__NODE__EXPRESSION__TOREGISTER__REGISTER__REGISTER_H__
#define __TOYLANG__NODE__EXPRESSION__TOREGISTER__REGISTER__REGISTER_H__
#include "../tempRegister.h"

typedef struct {
  TempRegisterNode base;
  char *name;
} VariableNode;

VariableNode *create_variable_node(char *name);

#endif
