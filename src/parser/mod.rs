mod expressions;
use core::panic;
use std::mem;

use expressions::*;

use crate::token::{Token, TokenType as Type};

// The main Interface/APi to interact with to start the parsing process.
//      from the tokens the lexer provides,
//      that are just in a flat row.
pub struct SyntaxTree {
    errs: Vec<String>, // Error messages TODO: implement when we can test
    root: Expr,
    is_eof: bool,
}
impl SyntaxTree {
    pub fn parse(tokens: Vec<Token>) {
        //let parser = Parser{};
        //return parser.Parse();
    }
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
    //TODO-check idea: combine the above into iterator? -> then add previous:Token and current:Token for example?
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
        // TODO DOES THIS REALLY WORK? CHECK ! *&&, && wtf?
        mem::discriminant(*&typ_check) == mem::discriminant(*&&typ)    // because String("1") != String("s") otherwise!
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
        if self.expect(vec![Type::Exclamation, Type::Minus]){
            return Expr::Unary(UnaryExpr { token: self.previous().typ.clone(), right: Box::new(self.unary()) })
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr{
        if self.expect(vec![Type::True]){
            return Expr::Literal(LiteralExpr::Boolean(true))
        }
        if self.expect(vec![Type::False]){
            return Expr::Literal(LiteralExpr::Boolean(false))
        }
        if self.expect(vec![Type::Nil]){
            return Expr::Literal(LiteralExpr::Nil)
        }

        if self.expect(vec![Type::Number(0.0)]){
            if let Type::Number(nr) = self.previous().typ{
                return Expr::Literal(LiteralExpr::Number(nr))
            }
        }
        if self.expect(vec![Type::String("".to_string())]){
            if let Type::String(str) = &self.previous().typ{
                return Expr::Literal(LiteralExpr::String(str.clone()))
            }
        }

        if self.expect(vec![Type::OpenParen]) {
            let expr = self.expression();   // back to the top and parse what is inside the parenthesis
            self.consume(Type::CloseParen, "Expect closing: ')' after expression.");   // need closing parenthesis
            return Expr::Grouping(GroupingExpr { expr: Box::new(expr) })
        }
        // TODO what to do here? Return None probably!
        panic!("END OF Parser.primary() reached WHEN IT SHOULD NOT HAVE, HELP!");
    }

    // 
    fn consume(&mut self, typ: Type, msg: &str) {
        if self.check(typ) {
            self.advance();
        }
        panic!("TODO: handle errors properly!")
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
