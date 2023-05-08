/*TokenType
    Scanner/Lexer/Tokenizer and adjacent functions/structs
*/
//      String              ->          Lexemes
// var language = "lox";    ->   [ var | language | = | "lox" | ; ]

use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::types::{Err, Token, TokenType, TokenType::*};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    source_arr: Vec<char>, // TODO remove?
    //iterator: CharIndices<'a>,    // iterator over all chars.
    tokens: Vec<Token<'a>>, // TODO: change to linkedList, maybe?
    errors: Vec<Err>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn new_scanner(source: &str) -> Scanner {
    let mut s = Scanner {
        source: source,
        source_arr: source.chars().collect(),
        //iterator: source.char_indices(),
        tokens: vec![],
        errors: vec![],
        start: 0,   // offsets that index into the string
        current: 0, // offsets that index into the string
        line: 1,
    };
    s.scan_all_tokens();
    s
}

impl<'a> Scanner<'a> {
    /// returns the created "Array" of Tokens to pass on to the Parser
    pub fn results(&self) -> (&Vec<Token>, Vec<Err>) {
        return (&self.tokens, self.errors.clone());
    }

    // "consumes" the Sourcecode to spit out tokens.
    fn scan_all_tokens(&mut self) {
        while !self.is_at_end() {
            // we are at the start of the next lexeme:
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            typ: EOF,
            lexeme: "",
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance_char();
        match c {
            // 1 Char long
            '(' => self.add_token(OpenParen),
            ')' => self.add_token(CloseParen),
            '{' => self.add_token(OpenBrace),
            '}' => self.add_token(CloseBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            // 1-2 char long combinations:
            '!' => match self.check_for('=') {
                true => self.add_token(ExclamationEqual),
                false => self.add_token(Exclamation),
            },
            '=' => match self.check_for('=') {
                true => self.add_token(EqualEqual),
                false => self.add_token(Equal),
            },
            '<' => match self.check_for('=') {
                true => self.add_token(LessEqual),
                false => self.add_token(Less),
            },
            '>' => match self.check_for('=') {
                true => self.add_token(GreaterEqual),
                false => self.add_token(Greater),
            },
            '/' => match self.check_for('/') {
                true => self.skip_line(),
                false => self.add_token(Slash),
            },
            // ignore whitespaces
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            // literals:
            '"' => self.string_literal(),
            _ => {
                if c.is_digit(10) {
                    // digit -> numbers
                    self.number_literal();
                } else if c.is_alphabetic() {
                    // word -> Identifier || Reserved-Word
                    self.identifier_literal();
                } else {
                    self.errors
                        .push(Err::Lexer("Unexpected character".to_string(), self.line));
                }
            }
        }
    }

    fn advance_char(&mut self) -> char {
        let ch = self.source_arr[self.current];
        self.current += 1;
        ch
    }

    fn add_token(&mut self, token: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token {
            typ: token,
            lexeme,
            line: self.line,
        });
    }

    // to check for 1-2 char long combinations. Ex: ! vs !=, < vs <=...
    fn check_for(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source_arr[self.current] != expected) {
            return false;
        }
        self.current += 1;
        true
    }

    // peek into following char
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; // return EOF
        }
        self.source_arr[self.current]
    }

    fn peek_two(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'; // return EOF
        }
        self.source_arr[self.current + 1]
    }

    // skip line fully (after // comment)
    fn skip_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance_char();
        }
    }

    // consume characters untill we hit the closing "
    fn string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance_char();
        }
        if self.is_at_end() {
            self.errors
                .push(Err::Lexer("Unterminated string".to_string(), self.line));
            return;
        }
        self.advance_char(); // consume the closing "
        let string_value = &self.source[self.start + 1..self.current - 1]; // TODO: do those values line up?
        self.add_token(String(string_value.to_string()));
    }

    // consume characters formatted aaa.bb untill no more digits found (with one possible .)
    fn number_literal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance_char();
        }

        // look for fraction part:
        if self.peek() == '.' && self.peek_two().is_digit(10) {
            self.advance_char();
            while self.peek().is_digit(10) {
                self.advance_char(); // consume digits after . 12.xx
            }
        }
        let s = &self.source[self.start..self.current];
        let number = s.parse::<f64>().unwrap_or_else(|_| {
            self.errors.push(Err::Lexer(
                "Failed to Parse Number->Float, used default value 0.0 instead!".to_string(),
                self.line,
            ));
            return 0.0;
        });
        self.add_token(Number(number));
    }

    // identifiers or KEYWORDS, like: "var x12_d" "print some_string"
    fn identifier_literal(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance_char();
        }
        let text = &self.source[self.start..self.current];
        match KEYWORDS.get(text) {
            Some(token_type) => self.add_token(token_type.clone()), // isKeyword    like "return"
            None => self.add_token(Identifier),                     // isIdentifier like "some_var"
        }
    }
}

// static KEYWORDS hashmap of reserved keywords and mapping them to the enums
lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", And);
        map.insert("class", Class);
        map.insert("else", Else);
        map.insert("false", False);
        map.insert("for", For);
        map.insert("fun", Fun);
        map.insert("if", If);
        map.insert("nil", Nil);
        map.insert("or", Or);
        map.insert("print", Print);
        map.insert("return", Return);
        map.insert("super", Super);
        map.insert("this", This);
        map.insert("true", True);
        map.insert("var", Var);
        map.insert("while", While);
        map
    };
}

/*
    Some unit tests for the Lexer as a whole:
*/

#[cfg(test)]
mod tests {
    use super::*;

    // helpers for testing:
    fn _fake_token(lexeme: &str, token: TokenType) -> Token {
        Token {
            typ: token,
            lexeme,
            line: 1,
        }
    }
    fn _fake_data(data: Vec<(&str, TokenType)>) -> Vec<Token> {
        let eof = vec![_fake_token("", EOF)].into_iter();
        data.iter()
            .map(|(lex, tok)| _fake_token(lex, tok.clone()))
            .chain(eof)
            .collect()
    }
    fn _is_expected(input: &str, expected: Vec<(&str, TokenType)>) {
        let s = new_scanner(input);
        let (tokens, errs) = s.results();
        let expected = _fake_data(expected);
        assert!(errs.len() == 0);
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn one_chars_longs() {
        _is_expected("", vec![]); // only an eof token!
        _is_expected(";", vec![(";", Semicolon)]);
        _is_expected("-", vec![("-", Minus)]);
    }

    #[test]
    fn braces() {
        _is_expected(
            "{)(}",
            vec![
                ("{", OpenBrace),
                (")", CloseParen),
                ("(", OpenParen),
                ("}", CloseBrace),
            ],
        );
    }

    #[test]
    fn two_char_longs() {
        _is_expected(
            "1=2",
            vec![("1", Number(1.0)), ("=", Equal), ("2", Number(2.0))],
        );
        _is_expected(
            "true== false",
            vec![("true", True), ("==", EqualEqual), ("false", False)],
        );
        _is_expected(
            "-05 !=00232.5",
            vec![
                ("-", Minus),
                ("05", Number(5.0)),
                ("!=", ExclamationEqual),
                ("00232.5", Number(232.5)),
            ],
        );
        _is_expected(
            "true==!false",
            vec![
                ("true", True),
                ("==", EqualEqual),
                ("!", Exclamation),
                ("false", False),
            ],
        );
        _is_expected(
            "return 1 / 2 // somecomment is NOT token/ // all ignored",
            vec![
                ("return", Return),
                ("1", Number(1.0)),
                ("/", Slash),
                ("2", Number(2.0)),
            ],
        );
    }

    #[test]
    fn literals() {
        _is_expected(
            "var name_varname = 5.5; this.thisname++",
            vec![
                ("var", Var),
                ("name_varname", Identifier),
                ("=", Equal),
                ("5.5", Number(5.5)),
                (";", Semicolon),
                ("this", This),
                (".", Dot),
                ("thisname", Identifier),
                ("+", Plus),
                ("+", Plus),
            ],
        );
    }

    #[test]
    fn newlines_and_whitespace() {
        let s = new_scanner("var \n return \t \n //ignored +-+ \n ;");
        let (tokens, errs) = s.results();
        let expected = vec![
            Token {
                typ: Var,
                lexeme: "var",
                line: 1,
            },
            Token {
                typ: Return,
                lexeme: "return",
                line: 2,
            },
            Token {
                typ: Semicolon,
                lexeme: ";",
                line: 4,
            },
            Token {
                typ: EOF,
                lexeme: "",
                line: 4,
            },
        ];
        assert!(errs.len() == 0);
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn unterminated_string_error() {
        _is_expected(
            " var parser = \"working\" ;",
            vec![
                ("var", Var),
                ("parser", Identifier),
                ("=", Equal),
                ("\"working\"", String("working".to_string())),
                (";", Semicolon),
            ],
        );
        // now without closing the string:
        let s = new_scanner(
            " var parser = \"notworking ; nothing after unterminated quotes should get reached",
        );
        let (tokens, errs) = s.results();
        let expected = _fake_data(vec![("var", Var), ("parser", Identifier), ("=", Equal)]);

        assert_eq!(1, errs.len());
        assert_eq!(tokens, &expected);
    }
    #[test]
    fn unexpected_character_error() {
        let s = new_scanner("#? v_a_r_ok _varNotOk");
        let (_tokens, errs) = s.results();
        assert_eq!(3, errs.len());
    }
}
