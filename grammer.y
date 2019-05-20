%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "src/tools/tools.h"
#include "src/ast/node.h"
#include "src/ast/expression/lvalue/variableReference/variableReference.h"
#include "src/ast/expression/rvalue/binaryOperationResult/binaryOperationResult.h"
#include "src/ast/expression/rvalue/intLiteral/intLiteral.h"
#include "src/ast/expression/rvalue/doubleLiteral/doubleLiteral.h"
#include "src/ast/statement/compound/compound.h"
#include "src/ast/statement/assign/assign.h"
#include "src/ast/statement/declare/declare.h"
#include "src/ast/statement/if/if.h"
#include "src/ast/statement/while/while.h"
#include "src/ast/statement/print/print.h"
#include "src/symbolTable/symbolTable.h"
#include "src/symbolTable/symbol/type/symbolType.h"
#include "src/symbolTable/symbol/symbol.h"
#include "src/symbolTable/symbol/array/array.h"

ASTNode *result;

int yylex();
int yyerror(char *errMsg);
%}

%union {
    char char_value;
    char *string_value;
    int int_value;
    double double_value;
    size_t id_value;
    ASTNode *node;
    SymbolType type;
    char print;
};

%token <type> TYPE
%token <string_value> IDENTIFY STRING_LITERAL
%token <int_value> INT_LITERAL
%token <double_value> DOUBLE_LITERAL
%token <id_value> STRING INT DOUBLE
%token <id_value> IF ELSE WHILE FOR
%token <id_value> LESSEQ GREATEREQ EQUAL NONEQUAL
%token <id_value> AND OR
%token <void> PRINT

%left '>' '<' LESSEQ GREATEREQ EQUAL NONEQUAL
%left '+' '-'
%left '*' '/'

%type <node> statement assignStatement statementList block ifStatement whileStatement defineStatement
%type <node> program expression assign atomExpression unaryExpression binaryOrAtomExpression printStatement

%%
program:
    program statement           {add_statement((CompoundStatement *)$1,$2);}
    |                           {$$=(ASTNode*)create_compound_statement();result=$$;}
    ;

assign:
    '=' expression              {$$=$2;}
    ;

defineStatement:
    TYPE IDENTIFY ';'                       {
            Symbol* symbol=create_symbol(true,$1,$2);
            add_symbol(symbol);
            $$=(ASTNode *)create_declare_statement(symbol, NULL);
        }
    | TYPE IDENTIFY '[' INT_LITERAL ']' ';'    {
            Symbol* symbol=(Symbol*)create_array_symbol(true,$1,$2,(size_t)($4));
            add_symbol(symbol);
            $$=(ASTNode *)create_declare_statement(symbol, NULL);
        }
    | TYPE IDENTIFY assign ';'              {
            Symbol* symbol=create_symbol(true,$1,$2);
            add_symbol(symbol);
            VariableReference * ref = create_variable_reference(symbol);
            AssignStatement * initial = create_assign_statement((LValue*)ref, (RValue*)$3);
            $$=(ASTNode *)create_declare_statement(symbol, initial);
        }
    ;

assignStatement:
    IDENTIFY assign ';'                     {
            Symbol* symbol = get_symbol($1);
            VariableReference* ref=create_variable_reference(symbol);
            $$=(ASTNode*)create_assign_statement((LValue*)ref, (RValue*)$2);
        }
    | IDENTIFY '[' INT_LITERAL ']' assign ';'  {
            $$=(ASTNode*)create_assign_statement((LValue*)get_symbol($1), (RValue*)create_int_literal($3));
        }
    ;

statementList:
    statementList statement                 {add_statement((CompoundStatement *)$1,$2);}
    |                                       {$$=(ASTNode*)create_compound_statement();}
    ;

block:
    '{' {push_frame();} statementList '}'   {$$=$3;pop_frame();}
    ;

ifStatement:
    IF expression block                     {$$=(ASTNode*)create_if_statement($2,(CompoundStatement *)$3,NULL);}
    | IF expression block ELSE block        {$$=(ASTNode*)create_if_statement($2,(CompoundStatement *)$3,(CompoundStatement *)$5);}
    ;

whileStatement:
    WHILE expression block                  {$$=(ASTNode*)create_while_statement($2,(CompoundStatement *)$3);}
;

printStatement:
    PRINT '(' expression ')' ';'            {$$=(ASTNode*)create_print_statement((RValue*)$3);}
;

statement:
    defineStatement
    | assignStatement
    | block
    | ifStatement
    | whileStatement
    | printStatement
;

atomExpression:
    INT_LITERAL                             {$$=(ASTNode*)create_int_literal($1);}
    | DOUBLE_LITERAL                        {$$=(ASTNode*)create_double_literal($1);}
    | IDENTIFY                              {
            Symbol* symbol=get_symbol($1);
            $$ = (ASTNode*)create_variable_reference(symbol);
        }
    | STRING_LITERAL                        {}
    | '(' expression ')'                    {$$=$2;}
    ;

unaryOperator:
    '+'
    | '-'
    | '!'
    ;

unaryExpression:
    atomExpression                          {$$=$1;}
    | unaryOperator atomExpression          {$$=$2;}
;

binaryOrAtomExpression:
    unaryExpression                                     {$$=$1;}
    | binaryOrAtomExpression '+' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('+',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression '-' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('-',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression '*' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('*',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression '/' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('/',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression '<' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('<',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression '>' unaryExpression        {$$=(ASTNode*)create_binary_operation_result('>',(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression LESSEQ     unaryExpression {$$=(ASTNode*)create_binary_operation_result(LESSEQ   ,(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression GREATEREQ  unaryExpression {$$=(ASTNode*)create_binary_operation_result(GREATEREQ,(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression EQUAL      unaryExpression {$$=(ASTNode*)create_binary_operation_result(EQUAL    ,(RValue*)$1,(RValue*)$3);}
    | binaryOrAtomExpression NONEQUAL   unaryExpression {$$=(ASTNode*)create_binary_operation_result(NONEQUAL ,(RValue*)$1,(RValue*)$3);}
;

expression:
    binaryOrAtomExpression {}
    ;

%%

int yyerror(char *errMsg) {
    printf("%s", errMsg);
    return 0;
}

int main(int argc, char *argv[]) {
    push_frame();
    yyparse();
    if(argc == 1 || strcmp(argv[1],"-emit-llvm") == 0) {
        printf("@double_fmt_str = private unnamed_addr constant [4 x i8] c\"%%g\\0A\\00\", align 1\n"
               "@int_fmt_str = private unnamed_addr constant [4 x i8] c\"%%d\\0A\\00\", align 1\n");
        printf("define i32 @main() #0 {\n");
        result->generate_code(result);
        printf("ret i32 0\n");
        printf("}\ndeclare i32 @printf(i8*, ...) #1\n");
    } else if(strcmp(argv[1],"-emit-ast") == 0) {
        result->print_ast_node(result, 0);
    }
    pop_frame();
    return 0;
}
