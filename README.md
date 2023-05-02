# Writing an Interpreter with Rust
The book [Crafting Interpreters](https://craftinginterpreters.com/contents.html) by Robert Nystrom as a guid, i try to venture into a totally new field of Programming.
- Goal of this project is to write along in Rust, while reading trough the book.


## A rough guide
Roughly the path information "takes"
- Source-Code
- Scanning
- Tokens
- Parsing
- Syntax Tree
    - Analysis
    - OR Transpiling
- Intermediate Reprensations
- Code Generation
    - Bytecode
    - OR Machine-Code

## Front-end
### Scanning - Lexing - Tokenizing
Take the stream of characters and chunks them into `tokens`.
- Examples: `103` a number, `"hi!"` a string literal, `let` a identifier.
- The scanner often disregards insignificant input like whitespace or Commented lines.

### Parsing
The ability to compose larger expressions and statements out of smaller parts.

The parser takes the flat sequence of tokens and builds a `tree` (also `AST` or `syntax tree`) structure that mirrors the nested logic.

Another important part of the parser is reporting syntax errors upstream.

### Static Analysis
- the above 2 Steps are fairly similar, while implementation of the next steps depend on the the characteristics of each language.

The first step of most analysis is to do **bindings** and **resolution**.
For each **identifier** we find out where it is defined and wire those together. For this it might be necessary to check scope.

For statically typed languages next could come type checking. Check if all bindings and resulutions match up. If not report a type error.

All those above semantic insight needs to be stored somewhere. A few implementations:
- as **attributes** in the syntax tree itself.
- in a **symbol table**. A lookup table off the side. Keys to this tables could be the identifiers (namees of variables and declarations)
- transform the tree into a new data structure that directly expresses those semantics of code.

## Middle-end
### Intermediate representations IR
The middle might store the code in some intermediate representation. That is not tightly tied to the sourcecode nor the destination. It acts as a middle.

This way it becomes easier to write one Compiler targeting multiple Backends, targeting for example x86, ARM, x64 Architectures all at once.

- A few example styles of IR: “control flow graph”, “static single-assignment”, “continuation-passing style”, and “three-address code”.

### Optimization
Once the Users Programm is defined we can swap out different Parts that do the same but impliment it more efficiently.

- A simple example is constant folding. If Something always evaluates to the same value `let volume = 3/4 *3.41 * 100` we can compile it as `let volume = 255.75`
- A few kewords here: constant propagation, common subexpression elimination, loop invariant code motion, global value numbering, strength reduction, scalar replacement of aggregates, dead code elimination, loop unrolling.

## Back-end

### Code generation
Generating primitive assemply-like instractions a CPU runs.
- We can choose to target a real CPU (machine code) of targeted architecture.
- Or for a Virtual one (bytecode). Here the VM emulates a hypothetical chip supporting your virtual architecture at runtime. (java etc.)

### Runtime
- For machine code we can just run it on the target machine
- Otherwise we start the VM and load our bytecode into it.

For all but the low-level languages this usually includes a bunch of services that the language provides. Like automatic memory management & garbage collection. Dynamic Type checking at runtime and much more.
- in for example go this part gets implemented into every executable, in java, python or JS the VM-interpreter will handle those parts.

## Alternate Routes
- Tree walk interpreters: Some languages begin executing right after parsing the AST. Just walk the tree and evaluate each node on the go. This is usually slow and ineffective but easy to implement.
- Transpilers: ex. Typescipt to Javascript.
- Just in time compilation aka JIT: JVM or most JavaScript interpreters do this. More sofisticated JITs will insert profiling hooks into generated code and recompile performance critical hot spots with more advanced optimisations (while the programm is already running)


