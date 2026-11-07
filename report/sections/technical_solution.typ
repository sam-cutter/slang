= Technical solution

The source code for my interpreter can be found in full in @interpreter_code.

Below are some interesting sections of my code which implement particularly complex functionality.

#table(
  columns: (3fr, 4fr, 2fr),
  table.header([*File name*], [*Description*], [*Document section*]),
  [`src/lexer.rs`], [Implements the finite state machine for the lexer described in @design:lexalg.], [@interpreter_code:lexer],
  [`src/parser.rs`], [Implements recursive-descent parsing to produce an abstract syntax tree.], [@interpreter_code:parser],
  [`src/expression.rs`], [Implements recursive evaluation of expressions via a post-order traversal of the abstract syntax tree. Also implements some reference counting functionality.], [@interpreter_code:expression],
  [`src/statement.rs`], [Implements the execution of statements. Also implements some reference counting functionality.], [@interpreter_code:statement],
  [`src/heap/garbage_collected.rs`], [Implements mark-and-sweep garbage collection.], [@interpreter_code:garbage_collected],
)