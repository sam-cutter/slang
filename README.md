# slang
The source code and project report for my A-level Computer Science NEA.

## Grammar
This is the current grammar of slang.

```
program -> statement*

statement -> printStatement
           | variableDeclaration
           | ifStatement
           | block
           | expressionStatement

expressionStatement -> expression ";"

printStatement -> "print" expression ";"

variableDeclaration -> "let" IDENTIFIER ("=" expression)? ";"

ifStatement -> "if" expression block ("else" (block | ifStatement))?

block -> "{" statement* "}"

expression -> assignment

assignment -> IDENTIFIER "=" assignment
            | ternary

ternary -> logical ("?" logical ":" logical)?

logical -> equality (("&&" | "||") equality)*

equality -> comparison (("!=" | "==") comparison)*

comparison -> bitwise ((">" | ">=" | "<" | "<=") bitwise)*

bitwise -> term (("&" | "|") term)*

term -> factor (("+" | "-") factor)*

factor -> unary (("*" | "/") unary)*

unary -> ("!" | "-")? primary

primary -> "(" expression ")"
         | STRING
         | INTEGER
         | FLOAT
         | IDENTIFIER
         | "true" | "false"
```