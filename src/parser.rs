use std::mem;

use crate::{
    expressions::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    types::{Err, Token, TokenType as Type}, statements::{Statement},
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
    pub root: Vec<Statement>,
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
        for n in &self.root{
            let _ = write!(&mut str, "{}", n);
        }
        return str;
    }
}

struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
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

    fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        return statements
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
}


/* 
        Handling Statements 
*/

impl<'a> Parser<'a> {
    fn statement(&mut self) -> Statement {
        if self.expect(vec![Type::Print]) {
            return self.printStatement();
        }
        return self.expressionStatement();
    }

    fn printStatement(&mut self) -> Statement {
        let value: Expr = self.expression();
        //TODO: handle runtime errors here? result from consume?
        _ = self.consume(Type::Semicolon, "Expected ; after value.");
        //TODO: check if value is string in here?
        return Statement::PrintSt(value)
    }

    fn expressionStatement(&mut self) -> Statement {
        let expr: Expr = self.expression();
        _ = self.consume(Type::Semicolon, "Expected ; after value.");
        return Statement::ExprSt(expr)
    }
}

/* 
        Handling Expressions

The Grammar rules sorted by precedence:
 PrioToCheck:
            1       ==  !=              equality()      ex: true != false
            2       >   >=  <   <=      comparison()    ex: 3>2
            3       +   -               term()          ex: 1+2-3
            4       *   /               factor()        ex: 1*3  or 10/5
            5       -   !               unary()         ex: -(3) or !false
            6       ()  true,false...   primary()       ex: Number(1.2) or "string" or (...) or nil
*/

impl<'a> Parser<'a> {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        // TODO: refactor this with proper enums && equality-> Some(Expr) instead!
        while self.expect(vec![Type::ExclamationEqual, Type::EqualEqual]) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                token: self.previous().typ.clone(),
                right: Box::new(self.comparison()),
            });
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.expect(vec![
            Type::Greater,
            Type::GreaterEqual,
            Type::Less,
            Type::LessEqual,
        ]) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                token: self.previous().typ.clone(),
                right: Box::new(self.term()),
            });
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.expect(vec![Type::Minus, Type::Plus]) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                token: self.previous().typ.clone(),
                right: Box::new(self.factor()),
            });
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.expect(vec![Type::Slash, Type::Star]) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                token: self.previous().typ.clone(),
                right: Box::new(self.unary()),
            });
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.expect(vec![Type::Exclamation, Type::Minus]) {
            return Expr::Unary(UnaryExpr {
                token: self.previous().typ.clone(),
                right: Box::new(self.unary()),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        self.advance();
        match &self.previous().typ {
            Type::True => Expr::Literal(LiteralExpr::Boolean(true)),
            Type::False => Expr::Literal(LiteralExpr::Boolean(false)),
            Type::Nil => Expr::Literal(LiteralExpr::Nil),
            Type::Number(nr) => Expr::Literal(LiteralExpr::Number(*nr)),
            Type::String(st) => Expr::Literal(LiteralExpr::String(st.clone())),
            Type::OpenParen => {
                let expr = self.expression(); // back to the top and parse what is inside the parenthesis
                if let Err(e) =
                    self.consume(Type::CloseParen, "Expect closing: ')' after expression.")
                {
                    //panic!("here");
                    self.errors.push(e);
                }; // need closing parenthesis
                Expr::Grouping(GroupingExpr {
                    expr: Box::new(expr),
                })
            }

            _ => {
                // cant parse sucessuflly
                self.errors.push(Err::Parser(
                    "Unexpected token <".to_string()
                        + &self.previous().to_string()
                        + "> ! parser.primary() failed.",
                    self.peek().line,
                ));
                Expr::ErrorExpr
            }
        }
    }

    /// We expect the Type (advance and return expr if so). Ff not we return an error.
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
/*
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
            left: Box::new(Expr::Literal(LiteralExpr::Boolean(true))),
            token: TokenType::EqualEqual,
            right: Box::new(Expr::Literal(LiteralExpr::Boolean(false))),
        });
        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }

    #[test]
    fn integration_test_mult_before_add() {
        // AST: <1 + <2 * 3>>
        let s = new_scanner("1+2*3");
        let (tokens, lexer_errs) = s.results();
        assert!(lexer_errs.len() == 0);
        let ast = AST::new(tokens);

        let expected = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
            token: Type::Plus,
            right: Box::new(Expr::Binary(BinaryExpr {
                left: Box::new(Expr::Literal(LiteralExpr::Number(2.0))),
                token: Type::Star,
                right: Box::new(Expr::Literal(LiteralExpr::Number(3.0))),
            })),
        });

        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }

    #[test]
    fn integration_test_parenthesis() {
        // AST: <(<1 + 2>) * 3>
        let s = new_scanner("(1-2)/3");
        let (tokens, lexer_errs) = s.results();
        assert!(lexer_errs.len() == 0);
        let ast = AST::new(tokens);

        let expected = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Grouping(GroupingExpr {
                expr: Box::new(Expr::Binary(BinaryExpr {
                    left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
                    token: Type::Minus,
                    right: Box::new(Expr::Literal(LiteralExpr::Number(2.0))),
                })),
            })),
            token: Type::Slash,
            right: Box::new(Expr::Literal(LiteralExpr::Number(3.0))),
        });
        assert_eq!(ast.root, expected);
        assert!(ast.errors.len() == 0);
    }
}

*/