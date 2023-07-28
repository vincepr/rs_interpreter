use std::mem;

use crate::{
    expressions::{
        BinaryExpr, Expr, FnCallExpr, GroupingExpr, LogicalExpr, UnaryExpr, Value, VarAssignExpr,
        VarReadExpr,
    },
    statements::{Statement, FunctionStatement},
    types::{Err, Token, TokenType as Type},
};

/*
    The AST. Abstract-Syntax-Tree is the exposed part of this module.
        - holds Data representing the logical Syntax that make up our programm.
        - Tree Structure with different enum-expressions
*/

/// The main Interface/APi to interact with to start the parsing process.
#[derive(Debug)]
pub struct AST {
    pub errors: Vec<Err>,
    pub root: Vec<Result<Statement, Err>>,
}
impl AST {
    /// parses a new AST (Abstract-Syntax-Tree) from a flat array of Token provided by the lexer/scanner
    pub fn new(tokens: &Vec<Token>) -> AST {
        let mut new_ast = Self {
            errors: vec![],
            root: vec![],
        };
        new_ast.parse(tokens);
        new_ast
    }

    fn parse(&mut self, tokens: &Vec<Token>) {
        let mut parser = Parser::new(tokens);
        self.root = parser.parse();
        self.errors = parser.errors;
    }

    /// pretty-print a representation of the AST. "(1+3)*3" becomes <(<1 + 2>) * 3>
    pub fn print(&self) -> String {
        use std::fmt::Write;
        let mut str = String::new();
        for res in &self.root {
            match res {
                Ok(statement) => {
                    let _ = write!(&mut str, "{}", statement);
                }
                Result::Err(e) => {
                    let _ = write!(&mut str, "{}", e);
                }
            }
        }
        return str;
    }
}

struct Parser<'a> {
    /// List of all Tokens we parse
    tokens: &'a Vec<Token<'a>>,
    /// Index to current token
    current: usize,
    errors: Vec<Err>,
}
impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: vec![],
        }
    }

    fn parse(&mut self) -> Vec<Result<Statement, Err>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        return statements;
    }
}

/*
        Helpers
*/

impl<'a> Parser<'a> {
    // info about current token. (==the next to be parsed)
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == Type::EOF
    }

    /// consume current token(by incrementing current) and returns reference to it
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// checks type of current-token == args. does not consume token.
    fn check(&mut self, typ: Type) -> bool {
        if self.is_at_end() {
            return false;
        }
        let typ_check = &self.peek().typ;
        // TODO DOES THIS REALLY WORK? To-CHECK! *&&, && wtf?
        mem::discriminant(*&typ_check) == mem::discriminant(*&&typ) // because String("1") != String("s") otherwise!
    }

    /// also known as: match()
    /// - checks if current token of any provided types
    /// - consumes token uppon success
    /// - reports back success -> bool
    fn expect(&mut self, types: Vec<Type>) -> bool {
        for typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// pushes error msg to the stack of errors, also returns a Error-Expression
    fn error_expr(&mut self, msg: &str) -> Err {
        self.errors.push(Err::Parser(msg.into(), self.peek().line));
        Err::Parser(msg.into(), self.peek().line)
    }
}

/*
        Handling Statements
*/

impl<'a> Parser<'a> {
    fn declaration(&mut self) -> Result<Statement, Err> {
        if self.expect(vec![Type::Fun]) {
            return self.function();
        }
        if self.expect(vec![Type::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    /// var IDENTIFIER optionalINITIALVALUE ;
    fn var_declaration(&mut self) -> Result<Statement, Err> {
        let name: String;
        if let Ok(token) = self.consume(Type::Identifier, "Expected variable name after var") {
            name = token.lexeme.to_string();
        } else {
            return Err(Err::Parser("Expected variable name after var".into(), 8));
        }
        let mut initializer = Expr::Literal(Value::Nil); // null if not initialized
        if self.expect(vec![Type::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(Type::Semicolon, "Expect ';' after variable declaration")?;
        return Ok(Statement::VariableSt(name, initializer));
    }

    fn while_statement(&mut self) -> Result<Statement, Err> {
        self.consume(Type::OpenParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(Type::CloseParen, "Expect ')' after while-condition.")?;
        let body = self.statement()?;
        return Ok(Statement::While {
            condition: condition,
            body: Box::new(body),
        });
    }

    fn for_statement(&mut self) -> Result<Statement, Err> {
        // for(initializer; condition; increment){body}     'for(var i=0; i<10; i++){print i;}'
        self.consume(Type::OpenParen, "Expect '(' after 'for'.")?;
        // the optional initializer: ex 'var i=0;'
        let initializer: Option<Statement>;
        if self.expect(vec![Type::Semicolon]) {
            initializer = None;
        } else if self.expect(vec![Type::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }
        // the optional condition ex 'x<10'
        let mut condition = if !self.check(Type::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(Type::Semicolon, "Expect ';' after for-loop condition.")?;
        // the optional increment: 'i=i+1'
        let increment = if !self.check(Type::CloseParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(Type::CloseParen, "Expect ')' after for-loop clauses.")?;
        // the body enclosed in {...}
        let mut body = self.statement();

        // desugaring = rebuilding our for loop with existing while loop and var, assign, block etc:
        if let Some(increment) = increment {
            let artificial_body: Vec<Result<Statement, Err>> =
                vec![body, Ok(Statement::ExprSt(increment))];
            body = Ok(Statement::BlockSt(artificial_body));
        }
        if condition == None {
            condition = Some(Expr::Literal(Value::Boolean(true)));
        }
        // save to unwrap here since we know we guarded against it (we know they exist)
        body = Ok(Statement::While {
            condition: condition.unwrap(),
            body: Box::new(body.unwrap()),
        });
        if let Some(initializer) = initializer {
            body = Ok(Statement::BlockSt(vec![Ok(initializer), body]));
        }
        return body;
    }

    fn statement(&mut self) -> Result<Statement, Err> {
        if self.expect(vec![Type::For]) {
            return self.for_statement();
        }
        if self.expect(vec![Type::If]) {
            return self.if_statement();
        }
        if self.expect(vec![Type::Print]) {
            return self.print_statement();
        }
        if self.expect(vec![Type::Return]) {
            return self.return_statement();
        }
        if self.expect(vec![Type::While]) {
            return self.while_statement();
        }
        if self.expect(vec![Type::OpenBrace]) {
            return Ok(Statement::BlockSt(self.block()));
        }
        return self.expression_statement();
    }

    fn if_statement(&mut self) -> Result<Statement, Err> {
        _ = self.consume(Type::OpenParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        _ = self.consume(Type::CloseParen, "Expect ')' after 'if' condition.")?;

        let then_ = self.statement()?;
        if self.expect(vec![Type::Else]) {
            let else_ = Some(Box::new(self.statement()?));
            return Ok(Statement::IfSt {
                condition: condition,
                then_: Box::new(then_),
                else_: else_,
            });
        }
        return Ok(Statement::IfSt {
            condition: condition,
            then_: Box::new(then_),
            else_: None,
        });
    }

    fn print_statement(&mut self) -> Result<Statement, Err> {
        let value: Expr = self.expression()?;
        _ = self.consume(Type::Semicolon, "Expected ; after value.")?;
        return Ok(Statement::PrintSt(value));
    }
    
    fn return_statement(&mut self) -> Result<Statement, Err> {
        let keyword = self.previous().lexeme.to_string();
        let mut value = Expr::Literal(Value::Nil);
        if !self.check(Type::Semicolon) {
            value = self.expression()?;
        }
        self.consume(Type::Semicolon, "Expect ';' after return value.");
        return Ok(Statement::ReturnSt { keyword, value });
    }

    fn expression_statement(&mut self) -> Result<Statement, Err> {
        let expr: Expr = self.expression()?;
        _ = self.consume(Type::Semicolon, "Expected ; after value.")?;
        return Ok(Statement::ExprSt(expr));
    }

    fn function(&mut self) -> Result<Statement, Err> {
        let name = self.consume(Type::Identifier, "Expect function/method name.")?.lexeme.to_string();
        
        self.consume(Type::OpenParen, "Expect '(' after function/method name.");

        let mut params = Vec::new();
        if !self.check(Type::CloseParen) {
            loop {
                if params.len() >= 255 {
                    return Err(Err::Parser(
                        "Can't have more than 255 parameters.".into(),
                        self.peek().line,
                    ));
                }
                params.push(
                    self.consume(Type::Identifier, "Expect parameter name.")?
                        .lexeme
                        .to_string(),
                );

                if !self.expect(vec![Type::Comma]) {
                    break;
                } // do while
            }
        }
        self.consume(Type::CloseParen, "Expect ')' after parameters.");
        self.consume(Type::OpenBrace, "Expect '{' before function/method body.");
        let body = self.block();
        return Ok(Statement::FunctionSt(FunctionStatement{ name, params, body }) );

    }

    /// a new block/scope
    fn block(&mut self) -> Vec<Result<Statement, Err>> {
        let mut statements = Vec::<Result<Statement, Err>>::new();
        while !self.check(Type::CloseBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        if let Err(did_err) = self.consume(Type::CloseBrace, "Expect '}' after block.") {
            statements.push(Err(did_err));
        }
        return statements;
    }
}

/*
        Handling Expressions

The Grammar rules sorted by precedence:
 PrioToCheck:
    expression ->   assignment;
    assignment ->   IDENTIFIER "=" assignment | equality();
            1       ==  !=              equality()      ex: true != false
            2       >   >=  <   <=      comparison()    ex: 3>2
            3       +   -               term()          ex: 1+2-3
            4       *   /               factor()        ex: 1*3  or 10/5
            5       -   !               unary()         ex: -(3) or !false
            6       ()  true,false...   primary()       ex: Number(1.2) or "string" or (...) or nil
*/

impl<'a> Parser<'a> {
    fn expression(&mut self) -> Result<Expr, Err> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Err> {
        let expr = self.logical_or();
        // we parse left side, if next is '=' then we know we are trying to assign:
        if self.expect(vec![Type::Equal]) {
            //let equals = self.previous();
            let value = self.assignment();
            if let Expr::VarRead(var) = expr? {
                let name = var.name;
                return Ok(Expr::VarAssign(VarAssignExpr::new(name, value?)));
            }
            return Err(self.error_expr("Invalid assignment target."));
        }
        return expr;
    }

    fn logical_or(&mut self) -> Result<Expr, Err> {
        let mut expr = self.logical_and();
        while self.expect(vec![Type::Or]) {
            let token = self.previous().typ.clone();
            let right = self.logical_and();
            expr = Ok(Expr::Logical(LogicalExpr {
                left: Box::new(expr?),
                token: token,
                right: Box::new(right?),
            }))
        }
        return expr;
    }

    fn logical_and(&mut self) -> Result<Expr, Err> {
        let mut expr = self.equality();
        while self.expect(vec![Type::And]) {
            let token = self.previous().typ.clone();
            let right = self.equality();
            expr = Ok(Expr::Logical(LogicalExpr {
                left: Box::new(expr?),
                token: token,
                right: Box::new(right?),
            }))
        }
        return expr;
    }

    fn equality(&mut self) -> Result<Expr, Err> {
        let mut expr = self.comparison();
        while self.expect(vec![Type::ExclamationEqual, Type::EqualEqual]) {
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr?),
                token: self.previous().typ.clone(),
                right: Box::new(self.comparison()?),
            }));
        }
        expr
    }

    fn comparison(&mut self) -> Result<Expr, Err> {
        let mut expr = self.term();

        while self.expect(vec![
            Type::Greater,
            Type::GreaterEqual,
            Type::Less,
            Type::LessEqual,
        ]) {
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr?),
                token: self.previous().typ.clone(),
                right: Box::new(self.term()?),
            }));
        }
        expr
    }

    fn term(&mut self) -> Result<Expr, Err> {
        let mut expr = self.factor();

        while self.expect(vec![Type::Minus, Type::Plus]) {
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr?),
                token: self.previous().typ.clone(),
                right: Box::new(self.factor()?),
            }));
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, Err> {
        let mut expr = self.unary();

        while self.expect(vec![Type::Slash, Type::Star]) {
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr?),
                token: self.previous().typ.clone(),
                right: Box::new(self.unary()?),
            }));
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, Err> {
        if self.expect(vec![Type::Exclamation, Type::Minus]) {
            return Ok(Expr::Unary(UnaryExpr {
                token: self.previous().typ.clone(),
                right: Box::new(self.unary()?),
            }));
        }
        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Err> {
        let mut arguments = Vec::new();
        if !self.check(Type::CloseParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Err::Parser(
                        "Can't have more than 255 arguments.".into(),
                        self.peek().line,
                    ));
                }
                arguments.push(self.expression()?);
                if !self.expect(vec![Type::Comma]) {
                    break;
                } // basically do while...
            }
        }
        let paren = self.consume(Type::CloseParen, "Expect ')' after arguments.")?;
        Ok(Expr::FnCall(FnCallExpr {
            callee: Box::new(callee),
            paren: paren.typ.clone(),
            arguments: arguments,
        }))
    }

    fn call(&mut self) -> Result<Expr, Err> {
        let mut expr = self.primary();
        loop {
            if self.expect(vec![Type::OpenParen]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }
        expr
    }

    fn primary(&mut self) -> Result<Expr, Err> {
        self.advance();
        match &self.previous().typ {
            Type::True => Ok(Expr::Literal(Value::Boolean(true))),
            Type::False => Ok(Expr::Literal(Value::Boolean(false))),
            Type::Nil => Ok(Expr::Literal(Value::Nil)),
            Type::Number(nr) => Ok(Expr::Literal(Value::Number(*nr))),
            Type::String(st) => Ok(Expr::Literal(Value::String(st.clone()))),
            Type::OpenParen => {
                let expr = self.expression(); // back to the top and parse what is inside the parenthesis
                if let Err(e) =
                    self.consume(Type::CloseParen, "Expect closing: ')' after expression.")
                {
                    self.errors.push(e.clone());
                    return Err(e);
                };
                Ok(Expr::Grouping(GroupingExpr {
                    expr: Box::new(expr?),
                }))
            }
            Type::Identifier => Ok(Expr::VarRead(VarReadExpr {
                name: self.previous().lexeme.to_string(),
            })),

            _ => {
                // cant parse sucessuflly
                self.errors.push(Err::Parser(
                    "Unexpected token <".to_string()
                        + &self.previous().to_string()
                        + "> ! parser.primary() failed.",
                    self.peek().line,
                ));
                return Err(Err::Parser(
                    "Unexpected token <".to_string()
                        + &self.previous().to_string()
                        + "> ! parser.primary() failed.",
                    self.peek().line,
                ));
            }
        }
    }

    /// We expect the Type (advance and return expr if so). If not we return an error.
    fn consume(&mut self, typ: Type, msg: &str) -> Result<&Token, Err> {
        // TODO if we actually use ErrorExpr instead of Result<Expr>
        // we should self.errors.push(e) in here not upstream!
        // and then just return a Option<&Token>
        match self.check(typ) {
            true => Ok(self.advance()),
            false => Err(Err::Parser(msg.to_string(), self.peek().line)),
        }
    }
}

/*
    Testing:
*/

#[cfg(test)]
mod tests {

    use crate::lexer::new_scanner;
    use crate::types::TokenType;

    use super::*;

    // helper for testing:
    fn _fake_tokens(data: Vec<(&str, TokenType)>) -> Vec<Token> {
        data.iter()
            .map(|(lexeme, typ)| Token {
                typ: typ.clone(),
                lexeme,
                line: 1,
            })
            .chain(vec![Token {
                typ: TokenType::EOF,
                lexeme: "",
                line: 1,
            }])
            .collect()
    }

    #[test]
    fn integration_test_with_lexer() {
        // AST: <true == false>
        let s = new_scanner("true == false;");
        let (tokens, lexer_errs) = s.results();
        assert!(lexer_errs.len() == 0);
        let ast = AST::new(tokens);

        let expected = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(Value::Boolean(true))),
            token: TokenType::EqualEqual,
            right: Box::new(Expr::Literal(Value::Boolean(false))),
        });

        let expected = vec![Ok(Statement::ExprSt(expected))];
        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }

    #[test]
    fn integration_test_mult_before_add() {
        // AST: <1 + <2 * 3>>
        let s = new_scanner("1+2*3;");
        let (tokens, lexer_errs) = s.results();
        assert!(lexer_errs.len() == 0);
        let ast = AST::new(tokens);

        let expected = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(Value::Number(1.0))),
            token: Type::Plus,
            right: Box::new(Expr::Binary(BinaryExpr {
                left: Box::new(Expr::Literal(Value::Number(2.0))),
                token: Type::Star,
                right: Box::new(Expr::Literal(Value::Number(3.0))),
            })),
        });

        let expected = vec![Ok(Statement::ExprSt(expected))];
        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }

    #[test]
    fn integration_test_parenthesis() {
        // AST: <(<1 + 2>) * 3>
        let s = new_scanner("(1-2)/3;");
        let (tokens, lexer_errs) = s.results();
        assert!(lexer_errs.len() == 0);
        let ast = AST::new(tokens);

        let expected = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Grouping(GroupingExpr {
                expr: Box::new(Expr::Binary(BinaryExpr {
                    left: Box::new(Expr::Literal(Value::Number(1.0))),
                    token: Type::Minus,
                    right: Box::new(Expr::Literal(Value::Number(2.0))),
                })),
            })),
            token: Type::Slash,
            right: Box::new(Expr::Literal(Value::Number(3.0))),
        });

        let expected = vec![Ok(Statement::ExprSt(expected))];
        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }
}
