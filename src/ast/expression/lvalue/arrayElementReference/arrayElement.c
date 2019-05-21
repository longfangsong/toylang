#include "arrayElement.h"
#include <stdio.h>

size_t next_pointer_register_id = 0;

#ifdef DEBUG

static void print_ast_node(ArrayElementReference *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("ArrayElementReference: %s\n", ((Symbol *) (node->array))->name);
}

#endif

static char *lvalue_ir(ArrayElementReference *lValue) {
    char *result = malloc(128);
    sprintf(result, "%%%s_element_%zu", ((Symbol *) (lValue->array))->name,
            lValue->lvalue_register_id);
    return result;
}

static char *rvalue_ir(ArrayElementReference *rValue) {
    char *result = malloc(128);
    sprintf(result, "%%temp_%zu", rValue->rvalue_register_id);
    return result;
}

static void generate_rvalue_code(ArrayElementReference *node) {
    char *lvalue_ir_str = ((LValue *) (node))->lvalue_ir((LValue *) (node));
    char *rvalue_ir_str = ((RValue *) (node))->rvalue_ir((RValue *) (node));
    ((LValue *) node)->generate_lvalue_code((LValue *) node);
    printf("%s = load %s, %s* %s\n",
           rvalue_ir_str,
           type_string(node->array->elementType),
           type_string(node->array->elementType),
           lvalue_ir_str);
    free(lvalue_ir_str);
    free(rvalue_ir_str);
}

static void generate_lvalue_code(ArrayElementReference *node) {
    char *lvalue_ir_str = ((LValue *) (node))->lvalue_ir((LValue *) (node));
    ((RValue *) (node->index))->generate_rvalue_code((RValue *) (node->index));
    char *index_rvalue_ir_str = ((RValue *) (node->index))->rvalue_ir((RValue *) (node->index));
    printf("%s = getelementptr inbounds [%zu x %s], [%zu x %s]* %%%s_%zu, i64 0, i32 %s\n",
           lvalue_ir_str,
           node->array->length,
           type_string(node->array->elementType),
           node->array->length,
           type_string(node->array->elementType),
           ((Symbol *) (node->array))->name,
           ((Symbol *) (node->array))->namespace_id,
           index_rvalue_ir_str
    );
    free(lvalue_ir_str);
    free(index_rvalue_ir_str);
}

ArrayElementReference *create_array_element_reference(ArraySymbol *array, RValue *index) {
    ArrayElementReference *result = (ArrayElementReference *) malloc(sizeof(ArrayElementReference));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((RValue *) (result))->type = array->elementType;
    ((RValue *) (result))->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    ((LValue *) (result))->generate_lvalue_code = (void (*)(LValue *)) generate_lvalue_code;
    ((LValue *) (result))->lvalue_ir = (char *(*)(LValue *)) lvalue_ir;
    ((RValue *) (result))->rvalue_ir = (char *(*)(RValue *)) rvalue_ir;
    result->array = array;
    result->index = index;
    result->rvalue_register_id = next_pointer_register_id++;
    result->lvalue_register_id = next_pointer_register_id++;
    return result;
}
