use std::str::Chars;

//#[allow(dead_code)]
#[derive(Debug)]
enum Token<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Pound,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Illegal,
}

struct Tokens<'a> {
    chars: Chars<'a>,
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(source: &'a str) -> Tokens<'a> {
        Tokens {
            chars: source.chars(),
        }
    }
}

impl<'a> Tokens<'a> {
    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn consume_if_eq(&mut self, c: char) -> bool {
        match self.peek() {
            Some(n) if c == n => {
                self.chars.next().unwrap();
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.clone().nth(1)
    }

    fn source(&self) -> &'a str {
        self.chars.as_str()
    }

    fn finished(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn identifier(&mut self, first: char) -> Token<'a> {
        loop {
            match self.peek() {
                Some(c) if c.is_alphabetic() => {
                    self.consume().unwrap();
                }
                Some(_) => {
                    self.consume().unwrap();
                    return Token::Identifier("");
                },
                None => return Token::Identifier(""),
            }
        }
    }

    fn string(&mut self) -> Token<'a> {
        let source = self.source();
        let length = 0;
        loop {
            match self.peek() {
                Some('"') => {
                    self.consume().unwrap();
                    return Token::String("");
                }
                Some(_) => {
                    self.consume().unwrap();
                },
                // unterminated string
                None => return Token::Illegal,
            }
        }
    }

    fn number(&mut self, first: char) -> Token<'a> {
        loop {
            match self.peek() {
                Some('"') => {
                    self.consume().unwrap();
                    return Token::String("");
                }
                Some(_) => {
                    self.consume().unwrap();
                },
                // unterminated string
                None => return Token::Illegal,
            }
        }
   }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => {
                    self.consume().unwrap();
                }
                Some('#') => self.skip_line(),
                _ => break,
            };
        }
    }

    fn skip_line(&mut self) {
        loop {
            let c = self.consume();
            match c {
                Some(c) if c == '\n' => break,
                _ => (),
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace_and_comments();

        let token = match self.consume()? {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '/' => Token::Slash,
            '*' => Token::Star,
            '#' => Token::Pound,
            '!' if self.consume_if_eq('=') => Token::BangEqual,
            '!' => Token::Bang,
            '=' if self.consume_if_eq('=') => Token::EqualEqual,
            '=' => Token::Equal,
            '<' if self.consume_if_eq('=') => Token::LessEqual,
            '<' => Token::Less,
            '>' if self.consume_if_eq('=') => Token::GreaterEqual,
            '>' => Token::Greater,
            '"' => self.string(),
            c @ '0'..='9' => self.number(c),
            c @ 'a'..='z' | c @ 'A'..='Z' => self.identifier(c),
            _ => Token::Illegal,
        };
        Some(token)
    }
}

pub fn compile(source: &str) {
    let tokens = Tokens::from(source);
    //    let mut prev_line = 0;

    for token in tokens {
        //        if token.line == prev_line {
        //            print!("{:>4} ", token.line);
        //        } else {
        //            print!("   | ");
        //        }
        println!("{:?}", token);
        //        prev_line = token.line;
    }
}
