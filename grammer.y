%{
#include "node/statement/statementList/statementList.h"
#include "node/expression/tempRegister/binaryOperator/binaryOperator.h"
#include "node/expression/tempRegister/variable/variable.h"
#include "node/statement/assign/assign.h"
#include "node/statement/declare/declare.h"
#include "node/statement/if/if.h"
#include "node/statement/while/while.h"
#include <stdio.h>
#define YYSTYPE ASTNode*
YYSTYPE result;
int yylex();
int yyerror(char* errMsg);
%}


%token VAR
%token CONST_VALUE
%token VARIABLE
%token IF
%token WHILE
%token EQUAL NONEQUAL


%left '>' '<' EQUAL NONEQUAL
%left '+' '-'
%left '*' '/'

%%
program:
    program statement                       {push((StatementListNode *)$1,$2);}
    |                                       {$$=(ASTNode *)create_list_node();result=$$;}
;   

expression:
    CONST_VALUE                             {$$=$1;}
    | VARIABLE                              {$$=$1;}
    | '(' expression ')'                    {$$=$2;}
    | expression '+' expression             {$$=(ASTNode *)create_binary_op_node('+',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression '-' expression             {$$=(ASTNode *)create_binary_op_node('-',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression '*' expression             {$$=(ASTNode *)create_binary_op_node('*',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression '/' expression             {$$=(ASTNode *)create_binary_op_node('/',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression '>' expression             {$$=(ASTNode *)create_binary_op_node('>',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression '<' expression             {$$=(ASTNode *)create_binary_op_node('<',(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression EQUAL expression           {$$=(ASTNode *)create_binary_op_node(EQUAL,(ExpressionNode *)$1,(ExpressionNode *)$3);}
    | expression NONEQUAL expression        {$$=(ASTNode *)create_binary_op_node(NONEQUAL,(ExpressionNode *)$1,(ExpressionNode *)$3);}

;

variableDeclareStatement:
    VAR VARIABLE ';'                        {$$=(ASTNode *)create_declare_node((VariableNode *)$2);}
;

variableAssignStatement:
    VARIABLE '=' expression ';'             {$$=(ASTNode *)create_assign_node((VariableNode *)$1,(ExpressionNode *)$3);}
;

ifStatement:
    IF '(' expression ')' '{' block '}'     {$$=(ASTNode *)create_if_node((ExpressionNode *)$3,(StatementListNode *)$6);}
;

whileStatement:
    WHILE '(' expression ')' '{' block '}'  {$$=(ASTNode *)create_while_node((ExpressionNode *)$3,(StatementListNode *)$6);}
;

statement:
    variableDeclareStatement
    | variableAssignStatement
    | ifStatement
    | whileStatement
;

block:
    block statement                         {push((StatementListNode *)$1,$2);}
    |                                       {$$ = (ASTNode *)create_list_node();}
;

%%

int yyerror(char* errMsg) {
    printf("%s\n",errMsg);
    return 0;
}

int main() {
    printf("@.str = private unnamed_addr constant [3 x i8] c\"%%d\\00\", align 1\n");
    printf("define i32 @main() #0 {\n");
    yyparse();
    result->generate_code(result);
    printf("%%1 = load i32, i32* %%c\n%%2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.str, i32 0, i32 0), i32 %%1)\nret i32 0\n}\ndeclare i32 @printf(i8*, ...) #1");
    return 0;
}
