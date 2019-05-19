#ifndef TOYLANG_DOUBLE_LITERAL_H
#define TOYLANG_DOUBLE_LITERAL_H

#include "../rvalue.h"

typedef struct DoubleLiteral {
    RValue base;
    double value;
} DoubleLiteral;

DoubleLiteral *create_double_literal(double value);

#endif //TOYLANG_DOUBLE_LITERAL_H
