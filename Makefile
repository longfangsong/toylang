compiler: lex.yy.c y.tab.c $(shell find ./node -name '*.c')
	gcc -std=c11 lex.yy.c y.tab.c $(shell find ./node -name '*.c') -o compiler
lex.yy.c: lexical.l y.tab.h
	lex lexical.l
y.tab.h y.tab.c: grammer.y
	yacc -d grammer.y
test.ll: compiler test.tyl
	./compiler < test.tyl > test.ll
test: test.ll
	gcc test.ll -o test
clean:
	rm -rf compiler lex.yy.c y.tab.c y.tab.h *.ll test a.out *.s