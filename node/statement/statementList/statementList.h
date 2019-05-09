#ifndef __TOYLANG__NODE__STATEMENT__LIST__LIST_H__
#define __TOYLANG__NODE__STATEMENT__LIST__LIST_H__
#include "../statement.h"
#include <stdlib.h>
typedef struct StatementListNode {
  StatementNode base;
  size_t size;
  StatementNode **statements;
} StatementListNode;

StatementListNode *create_list_node();

void push(StatementListNode *node, StatementNode *other);

#endif
