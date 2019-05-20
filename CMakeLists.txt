cmake_minimum_required(VERSION 3.2)
project(toylang)
ADD_DEFINITIONS(-DDEBUG)
find_package(bison)
find_package(flex)
flex_target(LEXICAL lexical.l ${CMAKE_CURRENT_SOURCE_DIR}/lex.yy.c)
bison_target(GRAMMER grammer.y ${CMAKE_CURRENT_SOURCE_DIR}/y.tab.c DEFINES_FILE ${CMAKE_CURRENT_SOURCE_DIR}/y.tab.h)
add_executable(compiler
        y.tab.c lex.yy.c
        src/symbolTable/symbol/symbol.c src/symbolTable/symbol/symbol.h
        src/symbolTable/symbol/type/symbolType.h
        src/symbolTable/symbol/array/array.c src/symbolTable/symbol/array/array.h
        src/symbolTable/symbolTable.c src/symbolTable/symbolTable.h
        src/tools/tools.c src/tools/tools.h
        src/ast/statement/statement.h
        src/ast/node.h
        src/ast/statement/compound/compound.c
        src/ast/statement/compound/compound.h
        src/ast/statement/assign/assign.c src/ast/statement/assign/assign.h
        src/ast/expression/expression.h
        src/ast/expression/lvalue/lvalue.h
        src/ast/expression/rvalue/rvalue.h
        src/ast/expression/lvalue/variableReference/variableReference.c src/ast/expression/lvalue/variableReference/variableReference.h
        src/ast/expression/lvalue/arrayElementReference/arrayElement.c src/ast/expression/lvalue/arrayElementReference/arrayElement.h
        src/symbolTable/symbol/type/symbolTable.c
        src/ast/expression/rvalue/binaryOperationResult/binaryOperationResult.c src/ast/expression/rvalue/binaryOperationResult/binaryOperationResult.h
        src/ast/statement/declare/declare.c src/ast/statement/declare/declare.h
        src/ast/expression/rvalue/intLiteral/intLiteral.c src/ast/expression/rvalue/intLiteral/intLiteral.h
        src/ast/expression/rvalue/doubleLiteral/doubleLiteral.c src/ast/expression/rvalue/doubleLiteral/doubleLiteral.h
        src/ast/statement/if/if.c src/ast/statement/if/if.h src/ast/statement/while/while.c src/ast/statement/while/while.h)
