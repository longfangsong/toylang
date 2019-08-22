#include "variableReference.h"
#include <stdio.h>


#ifdef DEBUG

static void print_ast_node(VariableReference *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("VariableReference: %s %zu\n", node->variable->name, node->variable->namespace_id);
}

#endif

static char *lvalue_ir(VariableReference *rValue) {
    char *result = malloc(128);
    sprintf(result, "%%%s_%zu", rValue->variable->name, rValue->variable->namespace_id);
    return result;
}

static void generate_rvalue_code(VariableReference *rValue) {
    char *rvalue_ir_string = ((RValue *) rValue)->rvalue_ir((RValue *) rValue);
    char *lvalue_ir_string = ((LValue *) rValue)->lvalue_ir((LValue *) rValue);
    printf("%s = load %s, %s* %s\n",
           rvalue_ir_string,
           symbol_type(rValue->variable),
           symbol_type(rValue->variable),
           lvalue_ir_string);
    free(rvalue_ir_string);
    free(lvalue_ir_string);
}

static void generate_lvalue_code(VariableReference *lValue) {
}

static char *rvalue_ir(VariableReference *rValue) {
    char *result = malloc(128);
    sprintf(result, "%%temp_%zu", rValue->temp_register_id);
    return result;
}

VariableReference *create_variable_reference(Symbol *variable) {
    VariableReference *result = (VariableReference *) malloc(sizeof(VariableReference));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((RValue *) result)->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    ((RValue *) result)->rvalue_ir = (char *(*)(RValue *)) rvalue_ir;
    ((LValue *) result)->generate_lvalue_code = (void (*)(LValue *)) generate_lvalue_code;
    ((LValue *) result)->lvalue_ir = (char *(*)(LValue *)) lvalue_ir;
    result->variable = variable;
    result->temp_register_id = next_temp_register++;
    ((RValue *) result)->type = variable->type;
    return result;
}