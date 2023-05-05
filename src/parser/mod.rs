mod expressions;
use std::mem;

use expressions::*;

use crate::{
    token::{Token, TokenType as Type},
    Err,
};

// The main Interface/APi to interact with to start the parsing process.
// holds the Tree structure that represents our Code-Logic
#[derive(Debug)]
pub struct AST {
    pub errors: Vec<Err>, // Error messages TODO: implement when we can test
    root: Expr,
}
impl AST {
    /// parses a new AST (Abstract-Syntax-Tree) from a flat array of Token provided by the lexer/scanner
    pub fn new(tokens: &Vec<Token>) -> AST {
        let mut new_ast = Self {
            errors: vec![],
            root: Expr::ErrorExpr,
        };
        new_ast.parse(tokens);
        new_ast
    }
    fn parse(&mut self, tokens: &Vec<Token>) {
        let mut parser = Parser::new(tokens);
        self.root = parser.parse();
        self.errors = parser.errors;
    }
    /// print out a representation of the AST. for debuging etc.
    pub fn print(&self) -> String {
        return self.root.to_string();
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
    fn parse(&mut self) -> Expr {
        self.expression()
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
        The Grammar rules sorted by precedence
   PrioToCheck
        1       ==  !=
        2       >   >=  <   <=
        3       -   +               (as factor) ex -(3) -> -3
        check documentation/Parser.md for the full list.
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
                    self.errors.push(e);
                }; // need closing parenthesis
                Expr::Grouping(GroupingExpr {
                    expr: Box::new(expr),
                })
            }

            _ => Expr::ErrorExpr,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn testing() {}
}
