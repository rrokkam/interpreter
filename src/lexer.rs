use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
struct Token<'a> {
    kind: Kind,
    text: &'a str,
}

impl<'a> Token<'a> {
    fn new(kind: Kind, text: &'a str) -> Token<'a> {
        Token { kind, text }
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
enum Kind {
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Pound,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,
    Identifier, String, Number,
    And, Class, Else, False, For, Fun, If, Nil, Or, Print, Return, Super, This, True, Var, While,
    Error,
}

struct Tokens<'a> {
    source: &'a str,
    iter: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(source: &'a str) -> Tokens<'a> {
        let iter = source.chars().peekable();
        Tokens {
            source,
            iter,
            start: 0,
            current: 0,
        }
    }
}

impl<'a> Tokens<'a> {
    fn advance(&mut self) -> Option<char> {
        let ch = self.iter.next();

        // https://wduquette.github.io/parsing-strings-into-slices/
        if let Some(c) = ch {
            self.current += c.len_utf8();
        }

        ch
    }

    fn advance_while(&mut self, mut func: impl FnMut(char) -> bool) {
        loop {
            match self.iter.peek() {
                Some(&c) if func(c) => {
                    self.advance().unwrap();
                }
                _ => return,
            }
        }
    }

    fn matches(&mut self, expected: char, kind: Kind) -> Option<Kind> {
        match self.iter.peek()? {
            c if *c == expected => {
                self.advance().unwrap();
                Some(kind)
            }
            _ => None,
        }
    }

    fn identifier_or_keyword(&mut self) -> Kind {
        self.advance_while(|c| c.is_alphabetic());
        match &self.source[self.start..self.current] {
            "and" => Kind::And,
            "class" => Kind::Class,
            "else" => Kind::Else,
            "false" => Kind::False,
            "for" => Kind::For,
            "fun" => Kind::Fun,
            "if" => Kind::If,
            "nil" => Kind::Nil,
            "or" => Kind::Or,
            "print" => Kind::Print,
            "return" => Kind::Return,
            "super" => Kind::Super,
            "this" => Kind::This,
            "true" => Kind::True,
            "var" => Kind::Var,
            "while" => Kind::While,
            _ => Kind::Identifier
        }

    }

    fn string(&mut self) -> Kind {
        self.advance_while(|c| c != '"');
        if let Some('"') = self.advance() {
            Kind::String
        } else {
            Kind::Error
        }
    }

    fn number(&mut self) -> Kind {
        self.advance_while(|c| c.is_numeric());
        Kind::Number
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.iter.peek() {
                Some(c) if c.is_whitespace() => {
                    self.advance().unwrap();
                }
                Some('#') => self.skip_line(),
                _ => break,
            };
        }
        self.start = self.current;
    }

    fn skip_line(&mut self) {
        self.advance_while(|c| c != '\n');
        self.advance().unwrap();
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace_and_comments();

        let kind = match self.advance()? {
            '(' => Kind::LeftParen,
            ')' => Kind::RightParen,
            '{' => Kind::LeftBrace,
            '}' => Kind::RightBrace,
            ';' => Kind::Semicolon,
            ',' => Kind::Comma,
            '.' => Kind::Dot,
            '-' => Kind::Minus,
            '+' => Kind::Plus,
            '/' => Kind::Slash,
            '*' => Kind::Star,
            '#' => Kind::Pound,
            '!' => self.matches('=', Kind::BangEqual).unwrap_or(Kind::Bang),
            '=' => self.matches('=', Kind::EqualEqual).unwrap_or(Kind::Equal),
            '<' => self.matches('=', Kind::LessEqual).unwrap_or(Kind::Less),
            '>' => self
                .matches('=', Kind::GreaterEqual)
                .unwrap_or(Kind::Greater),
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier_or_keyword(),
            _ => Kind::Error,
        };

        let text = &self.source[self.start..self.current];
        self.start = self.current;

        Some(Token::new(kind, text))
    }
}

pub fn compile(source: &str) {
    for token in Tokens::from(source) {
        println!("{:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_tokens() {
        let source = " my ( < != for\n #   forest \n1234  whiler";
        let mut tokens = Tokens::from(source);
        assert_eq!(tokens.next(), Some(Token::new(Kind::Identifier, "my")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::LeftParen, "(")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::Less, "<")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::BangEqual, "!=")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::For, "for")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::Number, "1234")));
        assert_eq!(tokens.next(), Some(Token::new(Kind::Identifier, "whiler")));
        assert_eq!(tokens.next(), None);
    }
}
