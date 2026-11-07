#show: page

= Interpreter source code <interpreter_code>

#set heading(outlined: false)

The interpreter source code has the following directory structure:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── environment.rs
    ├── expression.rs
    ├── heap
    │   ├── garbage_collected.rs
    │   ├── mod.rs
    │   ├── naive.rs
    │   └── reference_counted.rs
    ├── lexer.rs
    ├── main.rs
    ├── parser.rs
    ├── source.rs
    ├── stack.rs
    ├── statement.rs
    ├── stats.rs
    ├── token.rs
    ├── token_stream.rs
    └── value.rs

3 directories, 18 files
```

== `Cargo.lock`
#raw(read("../source_code/interpreter/Cargo.lock"), lang: "toml")

== `Cargo.toml`
#raw(read("../source_code/interpreter/Cargo.toml"), lang: "toml")

== `src/heap/garbage_collected.rs` <interpreter_code:garbage_collected>
#raw(read("../source_code/interpreter/src/heap/garbage_collected.rs"), lang: "rust")

== `src/heap/mod.rs`
#raw(read("../source_code/interpreter/src/heap/mod.rs"), lang: "rust")

== `src/heap/naive.rs`
#raw(read("../source_code/interpreter/src/heap/naive.rs"), lang: "rust")

== `src/heap/reference_counted.rs`
#raw(read("../source_code/interpreter/src/heap/reference_counted.rs"), lang: "rust")

== `src/environment.rs`
#raw(read("../source_code/interpreter/src/environment.rs"), lang: "rust")

== `src/expression.rs` <interpreter_code:expression>
#raw(read("../source_code/interpreter/src/expression.rs"), lang: "rust")

== `src/lexer.rs` <interpreter_code:lexer>
#raw(read("../source_code/interpreter/src/lexer.rs"), lang: "rust")

== `src/main.rs`
#raw(read("../source_code/interpreter/src/environment.rs"), lang: "rust")

== `src/parser.rs` <interpreter_code:parser>
#raw(read("../source_code/interpreter/src/parser.rs"), lang: "rust")

== `src/source.rs`
#raw(read("../source_code/interpreter/src/source.rs"), lang: "rust")

== `src/stack.rs`
#raw(read("../source_code/interpreter/src/stack.rs"), lang: "rust")

== `src/statement.rs` <interpreter_code:statement>
#raw(read("../source_code/interpreter/src/statement.rs"), lang: "rust")

== `src/stats.rs`
#raw(read("../source_code/interpreter/src/stats.rs"), lang: "rust")

== `src/token_stream.rs`
#raw(read("../source_code/interpreter/src/token_stream.rs"), lang: "rust")

== `src/token.rs`
#raw(read("../source_code/interpreter/src/token.rs"), lang: "rust")

== `src/value.rs`
#raw(read("../source_code/interpreter/src/value.rs"), lang: "rust")