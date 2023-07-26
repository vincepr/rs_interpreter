# Writing an Interpreter with Rust
The book [Crafting Interpreters](https://craftinginterpreters.com/contents.html) by Robert Nystrom writes a Java AST-Interpreter for the lox-language.

- Goal of this project is to write all code in Rust instead of the Java used in the book.
- Learning Rust was the main goal.
- For the [Stack-Based-Interpreter in C](https://github.com/vincepr/c_compiler) i stayed close to the original lox-language and just built features on top of it. This version does stray far from the original-implementation. The C-version also has a wasm-in browser [web-version](https://vincepr.github.io/c_compiler/)

## usage:
```
make build          // builds the ./target/release/rs_interpreter binary
make run            // will build the binary then run the test.lox in the root folder
make test           // will run the cargo unittests and the python script for all .lox files in tests/*
```