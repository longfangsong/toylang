%{
#include "node/expression/constant/constant.h"
#include "node/expression/tempRegister/variable/variable.h"
#define YYSTYPE ASTNode*
#include "y.tab.h"

int yyerror(char* errMsg);
extern YYSTYPE yylval;
%}

%%
"if"                    {return IF;}
"while"                 {return WHILE;}
"var"                   {return VAR;}
[0-9]+                  {
    yylval = (ASTNode*)create_constant_node(atoi(yytext));
    return CONST_VALUE;
  }
[a-zA-z_][0-9a-zA-z_]*  {
    yylval = (ASTNode*)create_variable_node(yytext);
    return VARIABLE;
  }
"=="                    {return EQUAL;}
"!="                    {return NONEQUAL;}
[ \t]                   {}
.                       {return *yytext;}
%%

int yywrap() {
    return 1;
}