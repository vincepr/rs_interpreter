# The interpreter
## Evaluator
First step after creating the AST is to walk those Expressions. And check what Expressions evaluate down to smaller Pices. For example `1+2*3` will evaluate to plain `7`.

Like  `<1+<2*3>>` to `<1+6>` to `<7>`.

- in my solution this is acchievd by by implementing the **Evaluated** Trait in Rust, basically as an interface over the different Expression-Types.
- then those just cascade down, till they cant be split anymore. Like Literals, for example `Literal(7)` for `1+2*3`.

## Statements
Statements are similar to expressions.
But instead of evaluating to some value they deal things like:
    - binding identifiers(/names) to data and functions
    - state and or side effects
    - Represent blocks and local scope

1. Expression statement
    Place an expression where a statement is expected. Example a function call `dothings();` or . Must end with a `;` in Lox.
2. Print Statement
    Evaluates to an expression and displays the result to the user. `print "asd";` 
    - in a proper language this should probably be a std library function. But for practical purpose, and get to evaluate other features, it gets implemented like this in lox.

added grammar rules:
```
program   →     statement* EOF ;
statement →     exprStmt | printStmt ;
exprStmt  →     expression ";" ;
printStmt →     "print" expression ";" ; 
```
The operands for `+` for example are always 2 expressions. The body of a while loop is always a statement. Since those two Syntaxes are always disjoint, we can have 2 separate trees.

- So we add the Syntax tree to our interpreter.

### Declarations
Declarations like `var drinks = "coca cola";` added to our grammar. 

- Extra care is required to disallow `if true var x = 1;` since scope and usefulness of that would be questionable.
- also defining a variable without initializing: `var x;` should be possible.
```
program   →     statement* EOF ;
program   →     varDecl | statement ;
statement →     exprStmt | printStmt ;
...
```

#### Environment
Bindings that associate variable-identifiers with valures need to be stored somewhere. This is usually called **environment**.

```
statement →     exprSt | printSt | block;
block
block     →     "{" declaration "}"
```


