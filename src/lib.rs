/*
    The Tree-Walk Interpreter jlox
*/

use std::{io::{self, Read, Write}, string, fmt::{self, write}, str::CharIndices};

pub fn run_prompt(){
    println!("Interpreter running, input a line:");
    loop {
        // read user input
        print!(">");
        io::stdout().flush().expect("flush failed!");   // rust stdin is buffered so we have to flush it
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.starts_with("#exit"){
            break
        }
        run(input);
    }
}

pub fn run_file(s: String){
    println!("TODO: load from file {s}")
}

fn run(input: String){
    let mut scanner = new_scanner(&input);
    scanner.scan_all_tokens();
    dbg!(scanner);
    //scanner.tokens = scanner.scan_tokens();

    //TODO: error handling here
}



/*
    Scanner/Tokenizer and adjacent functions/structs
*/
//      String              ->          Lexemes
// var language = "lox";    ->   [ var | language | = | "lox" | ; ]

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
    // single-character tokens
    OpenParen, CloseParen, OpenBrace, CloseBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // 1-2 character tokens
    Exclamation, ExclamationEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier,
    String(String),
    Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
struct Err{
    line: usize,
    msg: &'static str,
}



// Our Scanner/Tokenizer implementation
#[derive(Debug)]
struct Token<'a>{
    typ: TokenType,
    lexeme: &'a str,
    line: usize,
    // char_nr: usize,
    // literal Literal,
}
impl fmt::Display for Token<'_>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "typ: <{}> lexeme: <{}> line:<{}>", self.typ, self.lexeme, self.line)
    }
}


#[derive(Debug)]
struct Scanner<'a>{
    source: &'a str,
    source_arr: Vec<char>,         // TODO remove?
    iterator: CharIndices<'a>,    // iterator over all chars.
    tokens: Vec<Token<'a>>,     // TODO: change to linkedList ?
    errors: Vec<Err>,
    start: usize,
    current: usize,
    line: usize,
}
fn new_scanner(source: &str)-> Scanner{
    Scanner {
        source: source,
        source_arr: source.chars().collect(),
        iterator: source.char_indices(), 
        tokens: vec![], 
        errors: vec![],
        start: 0,       // offsets that index into the string
        current: 0,     // offsets that index into the string
        line: 1,
    }
}

impl <'a>Scanner<'a>{
    // "consumes" the Sourcecode to spit out tokens.
    fn scan_all_tokens(& mut self){
        while (!self.is_at_end()){
            // we are at the start of the next lexeme:
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { typ: TokenType::EOF, lexeme: "", line: self.line});
    }
    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }
    fn scan_token(&mut self){
        let c = self.advance_char();
        dbg!(c);
        match c {
            // 1 Char long
            '(' => self.add_token(TokenType::OpenParen),
            ')' => self.add_token(TokenType::CloseParen),
            '{' => self.add_token(TokenType::OpenBrace),
            '}' => self.add_token(TokenType::CloseBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            // 1-2 char long
            '!' => match self.check_for('='){
                true => self.add_token(TokenType::Exclamation),
                false => self.add_token(TokenType::ExclamationEqual),
            },
            '=' => match self.check_for('='){
                true => self.add_token(TokenType::Equal),
                false => self.add_token(TokenType::EqualEqual),
            },
            '<' => match self.check_for('='){
                true => self.add_token(TokenType::Less),
                false => self.add_token(TokenType::LessEqual),
            },
            '>' => match self.check_for('='){
                true => self.add_token(TokenType::Greater),
                false => self.add_token(TokenType::GreaterEqual),
            },
            '/' => match self.check_for('/'){
                true => self.skip_line(),
                false => self.add_token(TokenType::Slash),
            },
            // ignore whitespaces
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            // literals:
            '"' => self.string_literal(),
            _ => {
                // check for digits
                if c.is_digit(10){
                    self.number_literal();
                } else{
                    self.errors.push(Err{line: self.line, msg: "Unexpected character"})
                }
            }
        }
    }
    fn advance_char(&mut self) -> char{
        let ch =  self.source_arr[self.current];
        self.current += 1;
        ch
    }
    fn add_token(&mut self, token: TokenType){
        // this whole text implementation sucks but i want it for debugging atm:
        let lexeme = &self.source[self.start .. self.current];
        self.tokens.push(Token { typ: token, lexeme, line: self.line});
    }

    // to check for 1-2 char long combinations. Ex: ! vs !=, < vs <=...
    fn check_for(&mut self, expected: char) -> bool{
        if self.is_at_end() || (self.source_arr[self.current] != expected) {
            return false;
        }
        self.current += 1;
        true
    }

    // peek into following char
    fn peek(&self) -> char {
        if self.is_at_end(){
            return '\0'    // return EOF
        } 
        return self.source_arr[self.current];
    }

    // skip line fully (after // comment)
    fn skip_line(&mut self){
        while self.peek() != '\n' && !self.is_at_end(){
            self.advance_char();
        }
    }

    // consume characters untill we hit the closing "
    fn string_literal(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n'{
                self.line += 1;
            }
            self.advance_char();
        }
        if self.is_at_end(){
            self.errors.push(Err{line: self.line, msg: "Unterminated string"});
            return;
        }
        self.advance_char();    // consume the closing "
        let string_value = &self.source[self.start+1..self.current-1]; // TODO: do those values line up?
        self.add_token(TokenType::String(string_value.to_string()));
    }

    fn number_literal(&mut self){
        while self.peek().is_digit(10){
            self.advance_char();
        }

        // Work in progress ... 
        // @ Number literals Chapter 4.6.2
    }
}