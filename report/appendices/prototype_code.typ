#show: page

= Prototype source code <prototype_code>

#set heading(outlined: false)

The prototype has the following directory structure.

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── expression.rs
    ├── lexer.rs
    ├── main.rs
    ├── parser.rs
    ├── source.rs
    ├── token.rs
    └── token_stream.rs

1 directory, 9 files
```

== `Cargo.lock`
#raw(read("../source_code/prototype/Cargo.lock"), lang: "toml")

== `Cargo.toml`
#raw(read("../source_code/prototype/Cargo.toml"), lang: "toml")

== `src/expression.rs` <prototype_code:evaluator>
#raw(read("../source_code/prototype/src/expression.rs"), lang: "rust")

== `src/lexer.rs` <prototype_code:lexer>
#raw(read("../source_code/prototype/src/lexer.rs"), lang: "rust")

== `src/main.rs`
#raw(read("../source_code/prototype/src/main.rs"), lang: "rust")

== `src/parser.rs` <prototype_code:parser>
#raw(read("../source_code/prototype/src/parser.rs"), lang: "rust")

== `src/source.rs` <prototype_code:source>
#raw(read("../source_code/prototype/src/source.rs"), lang: "rust")

== `src/token.rs` <prototype_code:token>
#raw(read("../source_code/prototype/src/token.rs"), lang: "rust")

== `src/token_stream.rs` <prototype_code:token_stream>
#raw(read("../source_code/prototype/src/token_stream.rs"), lang: "rust")