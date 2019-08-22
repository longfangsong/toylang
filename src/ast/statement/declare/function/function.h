#ifndef TOYLANG_FUNCTION_DECLARE_H
#define TOYLANG_FUNCTION_DECLARE_H

#include "../../statement.h"
#include "../../../../symbolTable/symbol/function/function.h"

typedef struct FunctionDeclareStatement {
    Statement base;
    Function *function;
} FunctionDeclareStatement;

FunctionDeclareStatement *create_function_declare(Function *function);

#endif //TOYLANG_FUNCTION_H
