= Design

== High-level overview
The purpose of the program is to read in user-written code, and then to attempt to execute this code, by performing successive transformations on the source code string, forming a structure which is much easier to execute. If, at any point, errors (which occurred as a result of user mistakes) occurred, they should be reported in a useful and informative format.

=== Key components

- The `Lexer` transforms the source code string (`Source`) into a sequence of tokens (`Token`).
- The `Parser` transforms the sequence of tokens (`TokenStream`) into an abstract syntax tree (encoded with `Statement` and `Expression`.)
- The execution stage executes statements (`Statement`) and evaluates expressions (`Expression`). It relies on the `Stack`, the `Environment`, and the `ManagedHeap`. Values resulting from evaluation of expressions are passed around in the `Value` data structure.
- Adjacent to the execution stage is memory management, which operates on the `ManagedHeap`. The `ManagedHeap` is either a `NaiveHeap`, a `ReferenceCountedHeap` or a `GarbageCollectedHeap`.
- Throughout the execution stage, execution statistics (`Entry`) are also logged by the `Logger`.
- The entire process is all controlled by the `main` function, which interacts with the user through the terminal, and calls the relevant components.

The diagram below shows how data flows through the system, and how the source code string is transformed into an abstract syntax tree.
#align(center, image("../images/Data flow.png"))

== Host programming language
I decided to write my interpreter using the Rust programming language, due to its advanced static type system, and its memory management model. The type system is _algebraic_, meaning that I can construct data structures using _sum_ types (```rust enum``` in Rust) and _product_ types (```rust struct``` in Rust), and then use ```rust match``` statements, to force me to consider all possible scenarios when writing code.

I am familiar with Rust, having used it in projects before, so it seemed like a suitable choice.

No external libraries (other than the Rust standard library) are required to write this interpreter.

== Lexing
Lexing is the process which transforms the source code string into a sequence of tokens, which are then used by the parser.

=== The lexing algorithm <design:lexalg>
The lexer can be modelled as a finite state machine, where each new character advances the lexer into a new state. If a character occurs which is not accounted for in the state transition diagram, the lexer transitions into an invalid state, and emits an error.

#figure(image("../images/Lexer state transition diagram.png"), caption: "The state transition diagram for my lexer.")

In the diagram above, a red arrow means that the source code string should be un-advanced. This isn't quite how the lexer is implemented in the code: in reality, the lexer is able to peek multiple characters into the future, in order to decide which state to transition into. It uses the `peek_after` method on the `Source` data structure to do this.

An arrow with no symbol attached to it can accept any character.

I have not included the tokens that the lexer emits on this diagram, as the diagram would become too complex and difficult to read.

The state transition diagram uses the longest token principle, where it always tries to consume the longest token before consuming the next token. For example, in the string `if x == 10`, the `==` is lexed as `DoubleEqual` rather than two of `Equal`.

=== The `Source` data structure
I decided to create a wrapper around the source code string, to consolidate all state into one place. The `Source` data structure depends on the `Location` data structure, in order to keep track of how much of the source code string has already been consumed.

#align(center, image("../images/Source and Location.png", width: 80%))

The data structure has a few useful helper methods:

- `peek` returns the next character, without advancing the position.
- `peek_after` returns the character after the next character, without advancing the position.
- `advance` returns the next character and advances the position.
- `matches` advances the position only if the next character is equal to a target. It also returns a Boolean representing whether the next character matched the target.
- `at_end` returns whether the entire source code string has been consumed.
- `location` returns the current location.

When the position is advanced, the `index` and `column` fields are incremented, and if the character is a newline character, then the `column` is reset to `1` and the `line` incremented.

=== The `Token` data structure
Tokens are what the lexer outputs. They represent the smallest useful unit of the language.

#align(center, image("../images/Token.png", width: 50%))

The `Token` structure requires on two ```rust enum```s: `TokenData` and `TokenKind`.

- `TokenData` represents all the possible types of token, as well as their associated data. As such, it has a variant for each type of token: `LeftParenthesis`, `RightParenthesis`, `LeftBrace`, `RightBrace`, `Comma`, `Dot`, `Semicolon`, `QuestionMark`, `Colon`, `Plus`, `Minus`, `Star`, `Slash`, `Exponent`, `Bang`, `BangEqual`, `Equal`, `DoubleEqual`, `Greater`, `GreaterEqual`, `Less`, `LessEqual`, `Ampersand`, `DoubleAmpersand`, `Pipe`, `DoublePipe`, `String(String)`, `Float(f64)`, `Integer(i32)`, `Boolean(bool)`, `If`, `Else`, `While`, `Return`, `Let`, `Fu`, `Identifier(String)`. Any variants denoted with brackets have associated data, and the type of that associated data is inside the brackets.
- `TokenKind` is a _shadow_ of `TokenData`, and has the exact same variants, just with no associated data.

The `TokenData` structure has a method to extract its `TokenKind`, which is useful in the parsing stage for pattern matching.

== Parsing

=== Recursive descent parsing algorithm
Recursive descent parsing attempts to transform a stream of tokens into an abstract syntax tree. It scans the list of tokens from left to right, consuming one token at a time.

The parser is designed around a grammar, which defines the syntax of the language. The grammar for Slang is given in @grammar, but is repeated below for convenience.

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

assignment -> (call ".")? IDENTIFIER "=" assignment
            | ternary

ternary -> logical ("?" logical ":" logical)?

logical -> equality (("&&" | "||") equality)*

equality -> comparison (("!=" | "==") comparison)*

comparison -> bitwise ((">" | ">=" | "<" | "<=") bitwise)*

bitwise -> term (("&" | "|") term)*

term -> factor (("+" | "-") factor)*

factor -> unary (("*" | "/") unary)*

unary -> ("!" | "-")? exponent

exponent -> call ("^" exponent)?

call -> primary ( ("(" (expression ("," expression)*)? ")") | ("." IDENTIFIER) )*

primary -> "(" expression ")"
         | STRING
         | INTEGER
         | FLOAT
         | IDENTIFIER
         | object
         | "true" | "false"

object -> "{" (IDENTIFIER ":" expression ("," IDENTIFIER ":" expression)*)? "}"
```

Each rule in the grammar is given in the format *non-terminal* `->` *production*. A _non-terminal_ is a syntactic variable which represents a concept in a programming language, for example `whileLoop`. The right-hand-side of the rule (after the `->`) is the production. A production is a way that this non-terminal can appear in the source code string. A non-terminal can have multiple productions, and these are separated by the `|` symbol, to represent a union. The metalanguage that my grammar is defined in supports the following operators:

- `|` means "or".
- `*` means "zero or more of".
- `+` means "one or more of".
- `?` means "zero or one of".
- `()` groups syntax together.
- `->` separates the left-hand-side from the right-hand-side.

Productions can contain other non-terminals (including themselves), and _terminals_. A terminal is an actual token.

An example of a rule is:

```
ifStatement -> "if" expression block ("else" (block | ifStatement))?
```

This reads as: `ifStatement` is a non-terminal, and is constructed as follows: the string `if` (a terminal), followed by an expression, and then a block. Optionally, an else-statement may be attached, which is constructed with: the string `else`, followed by either a block or an if-statement. As seen, this is a recursive production.

The parser starts by attempting to parse a 'start' symbol (in the case of Slang, `program`), and then decides which production to attempt to parse. For the grammar I wrote, the appropriate production can always be selected by looking at only the next token.

In code, I implement the parser by adding a subroutine for each rule. The code then looks very similar to the grammar itself, as the subroutine for each production simply calls the appropriate subroutines for other non-terminals, occasionally accessing the `TokenStream` to consume an actual terminal/token.

Continuing the example of `ifStatement`, a pseudocode version of its parsing algorithm looks like this:

#block(breakable: false)[
```
PROCEDURE IfStatement()
  Consume(If)

  condition <- Expression()

  executeIfTrue <- Block()

  executeIfFalse <- null

  IF Tokens.PeekKind() = Else
    Consume(Else)

    IF Tokens.PeekKind() = If
      executeIfFalse <- IfStatement()
    ELSE IF Tokens.PeekKind() = LeftBrace
      executeIfFalse <- Block()
    ELSE
      THROW ParserError
    ENDIF
  ENDIF

  RETURN {condition, executeIfTrue, executeIfFalse}
ENDPROCEDURE
```]

Here, `Consume` is a subroutine which consumes the next token, as long as its kind matches the given kind. This corresponds to `TokenStream::consume`. `Expression()` and `Block()` are subroutines which parse an expression and a block, respectively. `Tokens.PeekKind()` gets the kind of the next token; this (more or less) corresponds to `TokenStream::Matches` in the code.

Similar subroutines are written for each of the non-terminals.

One issue which is very important in parsing is error handling. I chose to use 'panic-mode', where, after an error is encountered, the parser blindly consumes tokens, until a token which signals a fresh start is encountered. In my program, these tokens are semicolons, and the keywords `if`, `fu`, `let` and `while`. An error is also emitted and logged. Choosing this methods allows more unique errors to be picked up, while also minimising 'cascaded' errors (errors which only appear as a result of a previous error).

=== The `TokenStream` data structure
The `TokenStream` structure is a wrapper around a list of tokens. It provides useful helper methods which allow the parser to do easy pattern matching.

#align(center, image("../images/TokenStream.png", width: 70%))

The underlying list of tokens is a `VecDeque`, which is a queue-like structure provided by the Rust standard library that allows pushing and popping from both ends of the list. This means that methods such as `advance` can consume tokens from the start of the list.

The `peek` method allows the parser to look at the next unconsumed token, without actually advancing the token stream. This method returns a reference to the token.

The `advance` method advances the token stream and returns the next unconsumed token.

The `only_take` method takes in a list of targets, each of which has type `TokenKind`. It advances the token stream and returns the next unconsumed token only if the next token has kind equal to one of the targets.

The `matches` method is very similar to `only_take`, apart from instead of returning the next unconsumed token, a Boolean value is returned indicating whether the token stream was advanced.

The `binary_operator` method takes in a list of targets, where each target is a `BinaryOperator`. If the next token is a binary operator and matches one of the targets, the token stream is advanced and the binary operator is returned.

The `unary_operator` method takes in a list of targets, where each target is a `UnaryOperator`. If the next token is a unary operator and matches one of the targets, the token stream is advanced and the unary operator is returned.

The `consume_identifier` method attempts to consume the next token, but will only do so if it is an identifier. If it is an identifier, then the string contained within it is returned, otherwise an error is returned.

The `consume` method attempts to consume the next token, but only if it has a kind equal to the target. An error is returned if the next token doesn't match the target.

The `at_end` method returns whether the token stream has been fully consumed.

=== The `Expression` data structure
The `Expression` structure is an ```rust enum``` which has a variant for every possible type of expression:

- `Ternary` stores a condition, a left operand and a right operand, each of which are expressions.
- `Binary` stores a left operand, an operator, and a right operand.
- `Unary`stores an operator and an operand.
- `Call` stores a function and a list of arguments.
- `Assignment` stores an identifier and a value to be assigned.
- `Grouping` stores a contained expression.
- `Literal` stores a value.
- `Variable`stores an identifier.
- `GetField` stores an expression representing an object, and a field identifier.
- `SetField` stores an expression representing an object, a field identifier, and a value expression to be assigned.
- `Object` stores a key-value structure mapping strings to expressions.

The main method on this structure is the `evaluate` method, which contains all of the code needed to evaluate any one of these expressions.

Two other structures (`BinaryOperator` and `UnaryOperator`) are used to represent the operations within the `Binary` and `Unary` variants of `Expression`.

=== The `Statement` data structure
The `Statement` structure is an ```rust enum``` which has a variant for every possible type of statement:

- `VariableDeclaration` represents a variable declaration, and stores an identifier and an (optional) expression.
- `IfStatement` represents an if-statement, and stores a condition (expression), and two statements representing the possible execution branches of the if-statement.
- `FunctionDefinition` represents a function definition, and stores an identifier, a list of parameter identifiers, and an expression representing the body of the function.
- `Return` stores an (optional) return value, and represents a return statement.
- `WhileLoop` stores a condition and an expression representing its body. It represents a while-loop.
- `Block` stores a list of statements. It can represent the body of an if-statement, while-loop or function.
- `Expression` stores a single expression, and represents an expression statement.

The main method on this structure is the `execute` method, which contains all of the code needed to execute any one of these statements.

== Execution/evaluation

=== The execution/evaluation algorithm
The execution/evaluation algorithm performs a post-order traversal of the abstract syntax tree. Each `Expression` node represents an expression to be evaluated, and each `Statement` node represents a statement to be executed.

The parser initially produces a list of statements to be executed. The execution algorithm begins by searching through all of these statements for function definitions, and executing these statements first, thus declaring all functions that could be used by the program. This ensures that functions can be used before they are declared in the code. The algorithm then executes each of the other statements in order.

The process for executing an individual statement is as follows:
- If the statement is a variable declaration, then evaluate the initialiser (if given), perform processes required for reference counting (detailed in @design:rc), then use the `Stack::top` method to get the most recent environment of the most recent stack frame, and then use `Environment::define` to define the variable with its initialiser. A `ControlFlow::Continue` is emitted.
- If the statement is a function definition, then use the `Stack::top` method to get the most recent environment of the most recent stack frame, and then use `Environment::define` to define the function. A `ControlFlow::Continue` is emitted.
- If the statement is an if-statement, evaluate the condition. If the condition is a Boolean true value, execute the block of the if-statement. If the condition is a Boolean false value, then execute the block of the else-statement (if given). Otherwise, emit an error.
- If the statement is a while-loop, continue to evaluate the condition, and while the condition is a Boolean true value execute the block of the while-loop. A `ControlFlow::Continue` is emitted when the condition evaluates to the Boolean false value.
- If the statement is a block, first enter a new scope using the `Stack::enter_scope` method, then execute all function definitions within the block. Afterwards, execute each of the other statements in sequence. If any of the statement executions emits a `ControlFlow::Break`, then stop executing the statements. Perform processes required for reference counting (detailed in @design:rc). Then exit the scope using `Stack::exit_scope`, and perform processes required for garbage collection (detailed in @design:gc). If any of the statements emitted a `ControlFlow::Break`, then that is emitted, otherwise a `ControlFlow::Continue` is emitted.
- If the statement is an expression statement, then evaluate the expression and emit a `ControlFlow::Continue`.
- If the statement is a return statement, then emit a `ControlFlow::Break` with the value of the evaluated expression.

In order to execute statements, expressions must also be evaluated.

- If the expression is a ternary expression, the condition is evaluated. If the condition evaluates to the Boolean true value, then the left operand is evaluated and returned. If the condition evaluates to the Boolean false value, then the right operand is evaluated and returned. Otherwise, an error is returned.
- If the expression is a binary expression, the left and right operands are evaluated, and the operation is applied to them, as long as operation is defined between those types. If not, an error is returned.
- If the expression is a unary expression, the operand is evaluated and the operation is applied to it, as long as the operation is defined on the type of the operand. If not, an error is returned.
- If the expression is a function call:
  - If the function is user-defined, the arguments are evaluated, a new stack frame is pushed, and then variables are defined, whose identifiers are the parameter name, and whose values are the values of the evaluated arguments. If the number of arguments does not match the number of parameters, an error is returned. The body of the function is then executed. Reference counting processes are then performed (detailed in @design:rc), and the stack frame is popped.
  - If the function is native, then the appropriate code is ran on the arguments to the function, depending on which native function has been called.
- If the expression is a grouping, the interior expression is evaluated and returned.
- If the expression is a literal, the interior value is returned.
- If the expression is a variable, the value of that variable is retrieved from the stack and returned. If the variable does not exist, an error is returned.
- If the expression is a field get, then the object expression is evaluated. If it is in fact an object, or object reference, then the field is looked up, and its value returned (or an error returned). If not, an error is returned.
- If the expression is a field set, the object expression is evaluated. If it is in fact an object, or an object reference, then the field is set, to the value of the evaluated operand, and the previous value (if there was one) is returned.
- If the expression is an object, an object is constructed with the evaluated fields, and returned.

Throughout this entire process, errors which occur during the evaluation and execution process are returned and propagated up the call stack.

As suggested by the algorithm, execution and evaluation lend themself to recursive implementation, which is how I decided to implement this.

=== Data structures to represent values
The `Value` structure is an ```rust enum``` which can represent all possible values within the language. It has a variant for each data type:

- The `String` variant stores a `String`, and represents a string.
- The `Float` variant stores an `f64`, and represents a floating point number.
- The `Integer` variant stores an `i32`, and represents an integer.
- The `Boolean` variant stores a `bool`, and represents a Boolean.
- The `Function` variant stores a `Function`, and represents a function (may be user-defined or native).
- The `ObjectReference` variant stores a `Pointer`, and represents a reference to an object.
- The `Object` variant stores an `Object`, and represents an object.

Each variant also has a corresponding variant in the `Type` structure. The `Type` structure is almost identical to the `Value` structure, but it is effectively just a flag indicating the type of a value, and doesn't store the value itself. `Object` and `ObjectReference` have the same type.

Functions are stored in a structure called `Function`, which is also an ```rust enum```. It has a variant for each type of function:

- The `UserDefined` variant represents a user-defined function. It has a `parameters` field, which is a list of the names of the parameters. It also has a `block` field, which is the body of the function.
- The `Native` variant represents a native function, which is stored as `NativeFunction`, which is also an ```rust enum```. It has a variant for each native function: `Print`, `Format`, `Input`, `Int`, `Float`.

=== The `Stack` data structure
The `Stack` is the data structure within which all variables are stored. The underlying list is implemented as `Vec`, a structure provided by the Rust standard library which is a list, but allows pushing and popping like a stack.

#align(center, image("../images/Stack.png", width: 60%))

Each stack frame is, within itself, another stack. This is because, within a function call, there may be multiple scopes (i.e. if there is an if-statement). Each function call gets its own stack frame, but each scope (any time there is a block, indicated by curly braces) gets its own _environment_.

The `top` method returns a reference to the innermost stack frame, meaning the most recently pushed stack frame.

The `enter_scope` method creates a new `Environment`, and links it to the most recent environment in the most recent stack frame. This method should be called any time an if-statement, while-loop etc. is entered.

The `add_returned_object_reference` adds a pointer to a special list stored inside the most recent environment in the most recent stack frame. The reason for this is explained in @design:rc.

The `exit_scope` method should be called whenever a scope is exited, for example when an if-statement is exited. It decrements all references stored in the most recent environment's `returned_object_references` list, and replaces the most recent environment with its parent. The way in which environments are chained together is explained in @design:env.

The `push` method pushes a new frame onto the stack, and returns a mutable reference to the new environment that is created inside of it. This should be called whenever a new function is entered.

The `pop` method pops a stack frame off of the stack.

The `roots` method gathers the roots from each stack frame in the stack. This is used within garbage collection.

The `frames_count` method returns the number of frames currently on the stack. This is useful for statistics tracking.

=== The `Environment` data structure <design:env>
`Environment` is a data structure which stores all of the variables for a single scope.

Environments are chained together, so that variables declared inside of enclosing scopes can still be accessed inside inner scopes. Chaining works as follows: each environment has a parent (unless it is the global scope). The environment stores a mutable reference to this parent. When a new scope is created within a stack frame (via the `Stack::enter_scope` method), a new scope is created, with its parent set to the most recent environment within that stack frame. When a new stack frame is created (and therefore a new scope is created for that stack frame), the new scope has its parent set to the global scope. This means that all scopes are somewhat connected, in a similar way to a tree, however instead of each parent referencing its children, each child references its parents.

Variables are stored in a key-value data structure (`HashMap`), which maps strings to values. The value is actually optional, as my language allows declaring variables without initialising them.

The `returned_object_references` stores all pointers which this environment has received from functions that have been called. This is necessary for reasons explained in @design:rc.

#align(center, image("../images/Environment.png", width: 70%))

The `define` method defines a new variable, and inserts it into the most recent scope.

The `assign` method assigns a value to an already-existing variable. In order to find the correct variable to mutate, it first checks its own key-value data structure, and if the variable is not present then it hands off the task to its parent. This process continues up the chain of environments, until either the variable is found, or the global scope is reached and the variable is not found, in which case an error is propagated back up. This method also returns the previous value stored in that variable (if there was one), which is helpful for memory management, specifically reference counting.

The `get` method retrieves the value stored within a variable. To find the variable, it uses the same process as `assign`, working its way up the chain of environments.

The `global` method finds the global scope by recursively traversing the chain of parents.

The `roots` method gathers all roots from itself, and all of its parents, by traversing the chain of parents up to the global scope. This is used in garbage collection.

The `returned_object_references` method returns a reference to the `returned_object_references` list. Used for reference counting.

The `add_returned_object_reference` method adds a new pointer to the `returned_object_references` list. Used for reference counting.

The `parent` method returns a mutable reference to the parent of the current environment.

The `values` method returns a list of all values stored within the current environment.

== Memory management

=== Associated data structures
The `ManagedHeap` data structure is a wrapper around all of the specific types of heap, and is implemented as an ```rust enum```. It is always either a `NaiveHeap`, a `ReferenceCountedHeap`, or a `GarbageCollectedHeap`.

All types of heap rely on `Pointer`, which is an alias for a special smart pointer provided by the Rust standard library which supplies shared ownership with interior mutability. The `Pointer` references `HeapObject`, which stores the object itself, along with metadata required for the memory management methods to work (`marked` for garbage collection, `reference_count` for reference counting).

Most methods on the various heaps are explained in detail below, but there are a few extra methods:

- `ReferenceCountedHeap::conditionally_decrement` is a wrapper around the `decrement` method, which accepts a `Value` instead of an `Object`, only attempting to decrement the reference count if the `Value` is an instance of an `ObjectReference`.
- The `objects_count` method is included on all heaps, and is used for statistics tracking.
- `ManagedHeap::get_technique_code` is used for debugging purposes, and returns a string: `rc` if reference counting is being used, `na` if naive allocation is being used, and `gc` if garbage collection is being used.

#align(center, "../images/Memory management.png")

=== Naive allocation
Naive allocation does not have any cleanup procedure whatsoever: memory for objects is allocated on the heap and never deallocated. The allocation algorithm (named `allocate` in the code) is as follows:

+ Iterate through each property, $p$ of the object, $X$.
+ If $p$ is of type object:
  + Allocate memory for $p$.
  + Replace $p$ with a pointer to the allocated memory for $p$.
+ Place $X$ onto the heap.
+ Return a pointer to $X$.

As no memory for objects is ever cleaned up, the heap continues to grow until the program is terminated.

=== Garbage collection <design:gc>
I decided to use mark-and-sweep garbage collection for my project, as it is relatively easy to understand, and is very robust.

The algorithm for allocating memory on the heap is identical to the naive allocation algorithm

The algorithm for mark-and-sweep is as follows:

#block(breakable: false)[
```
PROCEDURE Manage(roots)
  FOR EACH root IN roots
    Traverse(root)
  ENDFOR

  FOR object IN heap
    IF NOT object.IsMarked()
      REMOVE object FROM heap
    ENDIF
  ENDFOR

  FOR object IN heap
    object.UnMark()
  ENDFOR
ENDPROCEDURE

PROCEDURE Traverse(root)
  IF root.IsMarked() THEN RETURN

  root.Mark()

  FOR EACH property IN root
    IF property IS ObjectReference
      Traverse(property)
    ENDIF
  ENDFOR
ENDPROCEDURE
```]

In the code, `Manage` is called `manage`, and `Traverse` is called `traverse`.

The process starts from a list of roots, which are all of the directly reachable objects. For an object to be directly reachable means that there is a reference directly to that object that exists somewhere on the stack. To get all of these roots, the `roots` method on the `Stack` data structure is used.

After the roots are collected, each root is traversed individually. Traversing a root means "marking" it (implemented as updating a Boolean flag from false to true), then traversing each object that it references. This lends itself to a recursive implementation, which is what I did.

The manage procedure is called every time that a stack frame is exited. This is frequent enough to keep the size of the heap relatively small, but infrequent enough that it doesn't slow down execution of the program too much.

=== Reference counting <design:rc>
The algorithm for reference counting is very similar to the one used for naive allocation and garbage collection, but it has a small modification:

+ Iterate through each property, $p$ of the object, $X$.
+ If $p$ is of type object:
  + Allocate memory for $p$.
  + Replace $p$ with a pointer to the allocated memory for $p$.
+ If instead $p$ is of type object reference.
  + Increment the reference count for $p$.
+ Place $X$ onto the heap.
+ Return a pointer to $X$.

Whenever memory for a new object is allocated, the reference counts of any objects which this new object references, must be incremented.

The general principle for reference counting is that each object has a reference count, and this reference count is incremented whenever a new reference is created to that object, and decremented whenever that reference goes out of scope. When the reference count of an object drops to 0, then the memory for that object is deallocated, and all objects which that object references have their reference counts decremented. This lends itself to a recursive implementation.

The main complexity with reference counting is introduced by the number of different places that these increment and decrement procedures need to be called.

Incrementing the reference count needs to happen in the following scenarios:
- When assigning an object reference to a variable.
- When setting the value of a field in an object to an object reference.
- When assigning an object reference (which was passed through as an argument) to a parameter.
- When allocating memory for an object.
- When defining a variable, and initialising it to an object reference.
- When returning an object reference from a function.

Decrementing the reference count must happen in the following cases:
- When the reference count of an object drops to 0, so all objects it references must have their counts decremented.
- When exiting a stack frame, so all object references which were returned from functions called within this stack frame must be decremented (more on this below).
- When a variable is mutated, and the previous value was an object reference.
- When the value of a field on an object is updated, and the previous value was an object reference.
- When a stack frame is exited, so all variables which directly reference an object go out of scope.
- When a variable is redefined (shadowing), and the previous value was an object reference.
- When a scope is exited, so all variables which directly reference an object go out of scope.

There is a specific problem which arises from returning object references from functions. Within the function, $f$, which is returning the object reference $x$, $f$ cannot know how $x$ will be used: $x$ could be assigned to a variable in the calling function, $c$, or it could be discarded, like:

```
fu c() {
  f();
}

fu f() {
  let x = {};
  return x;
}
```

If $x$ is in fact used, then its reference count should somehow end up as 1, but if it is not used, then its reference count should end up as 0, so that its memory can be deallocated.

One solution to this problem would be to decrement the reference count of $x$ to 0 when $f$ returns, and then if $x$ is in fact used in $c$, then its reference count would be incremented back up to 1. Otherwise, it would stay at 0 and be deallocated. However, this doesn't quite work: if $x$ has its reference count decremented to 0 at any point, then it will be immediately deallocated and will not be usable.

The solution which I landed on, was to maintain a list $l$, within the stack frame of $c$, of all object references given to $c$ by functions it called (i.e. $f$). When $f$ returns, it increments the reference count of $x$, and then in $c$, $x$ is added to the list $l$. When $c$ returns, all object references within $l$ have their reference counts decremented. This ensures that object references returned from functions live for the correct amount of time.

== Statistics tracking
The statistics tracking module is essential for benchmarking the performance of the programming language, its associated memory management techniques, and the programs written in it.

=== Entry logging frequency
When deciding how frequently to log an entry, there were to main options: either logging an entry at regular time intervals (e.g. every millisecond), or logging an entry every time that some sort of checkpoint was passed (e.g. every time an expression is evaluated). I opted for logging an entry every time that a statement is executed, specifically before that statement is executed. I also log an entry after the entire program has finished executing.

=== The `Entry` data structure
This data structure represents a single entry into the statistics log, and is a simple record structure.

#align(center, image("../images/Entry.png", width: 50%))

The memory usage of the interpreter is measured in bytes. This field is optional, as it is sometimes not possible to retrieve the memory usage of the interpreter, namely if the operating system that the program is being run on is not Linux.

=== The `Logger` data structure
This data structure represents an ordered collection of entries. The program is able to add new entries through the `Logger` and this structure is responsible for writing its data to a CSV file.

#align(center, image("../images/Logger.png", width: 80%))

The `start` field represents the instant that the logger is created, and this field is initialised with a value when the `Logger` is created.

Every time the `new_entry` method is called, the `elapsed` field for the new `Entry` is calculated based on how much time has elapsed since `start`.

Memory usage can only be calculated when the host operating system is Linux, and is determined by accessing the `/proc/self/status` file, and looking for the line beginning with `VmRSS:`. The memory usage in this file is given in KB, so is multiplied by 1000 to convert into bytes.

When `new_entry` is called, a new `Entry` is created and pushed onto the end of the `new_entry` structure.

When `write_to_csv` is called, all data is written, one entry per line, to a CSV file which has the name `<source_code_filename>.csv`.

== User interface
The user interacts with the program through a command-line application. When the user runs the program, they must specify two pieces of information, which they communicate via command line arguments. Firstly, they must decide which memory management method they would like to use. This is either naive mode (`na`), reference counting (`rc`), or garbage collection (`gc`). They then must specify the name of a file they'd like to run. This argument can be left blank, in which case a REPL is opened.

Examples of valid commands are as follows:

- `slang na test.slang` - runs the file `test.slang` in naive mode.
- `slang rc` - opens a REPL in reference counting mode.
- `slang gc file.txt` - runs the file `file.txt` in garbage collection mode.

Both REPL mode and file mode use a common `run` function to run source code. The `run` function takes in the source code string to be ran, an instance of a `Stack`, an instance of a `ManagedHeap`, and an instance of a `Logger`. This allows REPL mode to maintain state between lines of code executed, as it can create a stack and heap first, and then pass a mutable reference to these into the `run` function.

=== REPL mode
REPL (Read-Evaluate-Print-Loop) mode works as follows:

+ A user types a single line of input.
+ If the user pressed `Ctrl+C`, then exit.
+ Otherwise, the line of code is executed.
+ Return to step 1.

Throughout the REPL, a persistent environment is maintained, meaning that if the user declares a variable in one line, they are able to use it in another. Also, any errors will not crash the REPL; the user will still be able to enter and execute lines of code. This includes lexer errors, parser errors, and errors which occur in evaluation.

=== File mode
In file mode, the contents of a file are read, and then executed in their entirety. Any errors will halt the execution of the file. In any case, the performance statistics will be written to a CSV file at the end of execution by the `Logger`.