#include "paramList.h"

ParamList *create_param_list() {
    ParamList *result = (ParamList *) malloc(sizeof(ParamList));
    result->length = 0;
    result->content = NULL;
    return result;
}

void add_param(ParamList *list, Symbol *param) {
    list->content = realloc(list->content, sizeof(Symbol *) * (list->length + 1));
    list->content[list->length] = param;
    ++list->length;
}

