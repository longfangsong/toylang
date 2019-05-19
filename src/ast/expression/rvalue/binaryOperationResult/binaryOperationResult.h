#ifndef TOYLANG_BINARYOPERATIONRESULT_H
#define TOYLANG_BINARYOPERATIONRESULT_H

#include "../rvalue.h"

typedef struct BinaryOperationResult {
    RValue base;
    size_t temp_register_id;
    size_t operator_id;
    RValue *lhs;
    RValue *rhs;
} BinaryOperationResult;

BinaryOperationResult *create_binary_operation_result(size_t operator_id, RValue *lhs, RValue *rhs);

#endif //TOYLANG_BINARYOPERATIONRESULT_H
