# slang
The source code and project report for my A-level Computer Science NEA.

## Grammar
This is the current grammar of slang.

```
program -> statement*

statement -> variableDeclaration
           | nonDeclaration

variableDeclaration -> "let" IDENTIFIER ("=" expression)? ";"

nonDeclaration -> expression ";"
                | "print" expression ";"
                | "{" statement* "}"

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