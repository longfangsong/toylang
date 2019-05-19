#include "arrayElement.h"
#include <stdio.h>

size_t next_pointer_register_id = 0;

#ifdef DEBUG

static void print_ast_node(ArrayElementReference *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("ArrayElementReference: %s[%zu]\n", ((Symbol *) (node->array))->name, node->index);
}

#endif

static char *lvalue_ir(ArrayElementReference *lValue) {
    char *result = malloc(128);
    sprintf(result, "%%%s_index_%zu", ((Symbol *) (lValue->array))->name, lValue->index);
    return result;
}

static void generate_rvalue_code(ArrayElementReference *node) {
    char *lvalue_ir_str = ((LValue *) (node))->lvalue_ir((LValue *) (node));
    char *rvalue_ir_str = ((RValue *) (node))->rvalue_ir((RValue *) (node));

    printf("%s = getelementptr inbounds [%zu x %s], [%zu x %s]* %s, i64 0, i64 %zu\n",
           lvalue_ir_str,
           node->array->length,
           type_string(node->array->elementType),
           node->array->length,
           type_string(node->array->elementType),
           ((Symbol *) (node))->name,
           node->index
    );
    printf("%s = load %s, %s* %s", rvalue_ir_str, type_string(node->array->elementType),
           type_string(node->array->elementType), lvalue_ir_str);
    free(lvalue_ir_str);
    free(rvalue_ir_str);
}

ArrayElementReference *create_array_element_reference(ArraySymbol *array, size_t index) {
    ArrayElementReference *result = (ArrayElementReference *) malloc(sizeof(ArrayElementReference));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((RValue *) (result))->type = array->elementType;
    ((RValue *) (result))->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    ((LValue *) (result))->lvalue_ir = (char *(*)(LValue *)) lvalue_ir;
    result->array = array;
    result->index = index;
    result->pointer_register_id = next_pointer_register_id++;
    return result;
}
