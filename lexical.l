%{
#include "src/ast/node.h"
#include "src/symbolTable/symbolTable.h"
#include "y.tab.h"
%}

%%
"char"                      {yylval.type=Char; return TYPE;}
"int"                       {yylval.type=Int; return TYPE;}
"double"                    {yylval.type=Double; return TYPE;}
"string"                    {yylval.type=String; return TYPE;}
"if"                        {return IF;}
"else"                      {return ELSE;}
"while"                     {return WHILE;}
"print"                     {return PRINT;}
"return"                    {return RETURN;}
([-])?[0-9]+                {yylval.int_value=atoi(yytext); return INT_LITERAL;}
([-])?[0-9]+\.[0-9]*        {yylval.double_value=atof(yytext); return DOUBLE_LITERAL;}
[a-zA-Z][0-9a-zA-Z_]*       {yylval.string_value=strdup(yytext); return IDENTIFY;}
"=="                        {return EQUAL;}
"!="                        {return NONEQUAL;}
"<"                         {return *yytext;}
"<="                        {return LESSEQ;}
">"                         {return *yytext;}
">="                        {return GREATEREQ;}
"+"                         {return *yytext;}
"-"                         {return *yytext;}
"*"                         {return *yytext;}
"/"                         {return *yytext;}
"["                         {return *yytext;}
"]"                         {return *yytext;}
[ \t]                       {}
.                           {return *yytext;}
%%

int yywrap() {
    return 1;
}
