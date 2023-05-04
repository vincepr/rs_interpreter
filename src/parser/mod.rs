mod expressions;
use core::panic;

use expressions::*;

use crate::token::{Token, TokenType};

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
        self.peek().typ == TokenType::EOF
    }


    /// consume current token(by incrementing current) and returns reference to it
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    /// returns true if token is of given type. does not consume token.
    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().typ == typ
    }

    /// also known as: match()
    /// - checks if current token of any provided types
    /// - consumes token uppon success
    /// - reports back success -> bool
    fn expect(&mut self, types: Vec<TokenType>) -> bool {
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
        while self.expect(vec![TokenType::ExclamationEqual, TokenType::EqualEqual]) {
            expr = Expr::Binary(BinaryExpr {
                left: Some(Box::new(expr)),
                token: self.previous().typ.clone(),
                right: Some(Box::new(self.comparison())),
            });
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.expect(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Expr::Binary(BinaryExpr {
                left: Some(Box::new(expr)),
                token: self.previous().typ.clone(),
                right: Some(Box::new(self.term())),
            });
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.expect(vec![TokenType::Minus, TokenType::Plus]) {
            expr = Expr::Binary(BinaryExpr {
                left: Some(Box::new(expr)),
                token: self.previous().typ.clone(),
                right: Some(Box::new(self.factor())),
            });
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.expect(vec![TokenType::Slash, TokenType::Star]) {
            expr = Expr::Binary(BinaryExpr {
                left: Some(Box::new(expr)),
                token: self.previous().typ.clone(),
                right: Some(Box::new(self.unary())),
            });
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.expect(vec![TokenType::Exclamation, TokenType::Minus]){
            return Expr::Unary(UnaryExpr { token: self.previous().typ.clone(), right: Some(Box::new(self.unary())) })
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr{
        if self.expect(vec![TokenType::True]){
            return Expr::Literal(LiteralExpr {  })
        }
        if self.expect(vec![TokenType::False]){
            return Expr::Literal(LiteralExpr {  })
        }
        if self.expect(vec![TokenType::Nil]){
            return Expr::Literal(LiteralExpr {  })
        }

        if self.expect(vec![TokenType::Number(()), TokenType::String(())]){
            return Expr::Literal(self.previous())
        }

        if self.expect(vec![TokenType::OpenParen]) {
            let expr = self.expression();   // back to the top and parse what is inside the parenthesis
            self.consume(TokenType::CloseParen, "Expect closing: ')' after expression.");   // need closing parenthesis
            return Expr::Grouping(GroupingExpr { expr: Some(Box::new(expr)) })
        }
        // TODO what to do here? Return None probably!
        panic!("END OF Parser.primary() reached WHEN IT SHOULD NOT HAVE, HELP!");
    }

    // 
    fn consume(&self) {
        if check(typ) {
            self.advance();
        }

    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tree = UnaryExpr {
            token: TokenType::And,
            right: None,
        };
    }
}
