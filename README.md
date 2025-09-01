# slang
The source code and project report for my A-level Computer Science NEA.

## Grammar
This is the current grammar of slang.

```
program -> statement*

statement -> variableDeclaration
           | functionDefinition
           | returnStatement
           | ifStatement
           | whileLoop
           | block
           | expressionStatement

expressionStatement -> expression ";"

variableDeclaration -> "let" IDENTIFIER ("=" expression)? ";"

functionDefinition -> "fu" IDENTIFIER "(" (IDENTIFIER ("," IDENTIFIER)*)? ")" block

returnStatement -> "return" expression? ";"

ifStatement -> "if" expression block ("else" (block | ifStatement))?

whileLoop -> "while" expression block

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

unary -> ("!" | "-")? call

call -> primary ( "(" (expression ("," expression)*)? ")" )*

primary -> "(" expression ")"
         | STRING
         | INTEGER
         | FLOAT
         | IDENTIFIER
         | object
         | "true" | "false"

object -> "{" (IDENTIFIER ":" expression ("," IDENTIFIER ":" expression)*)? "}"
```