#[derive(Debug)]
enum TokenType {
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
    Identifier,
    String,
    Number,

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

    Error,
}

struct Token<'a> {
    token_type: TokenType,
    token: &'a str,
    line: usize,
    start: usize,
}

struct Tokens<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
}

impl<'a> Tokens<'a> {
    fn skip_whitespace(&mut self) -> Option<()> {
        loop {
            let next = *self.iter.peek()?;
            if next == ' ' || next == '\t' || next == '\r' {
                self.iter.next()?;
            } else if next == '\n' {
                self.iter.next()?;
                self.line += 1;
            } else if next == '#' {
                loop {
                    if self.iter.next_if_eq(&'\n').is_some() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        Some(())
    }

    fn string(&mut self) -> Option<Token<'a>> {
        while *self.iter.peek().ok_or_else(|| return TokenType::Error).unwrap() != '"' {
            let next = *self.iter.peek()?;
            if next == '\n' {
                self.line += 1;
            }

            // Unterminated string
            self.iter.next().ok_or_else(|| return TokenType::Error).unwrap();
        }
        Some(Token { token_type: TokenType::String, token: "", line: 0, start: 0 } )
    }

    fn number(&mut self) -> Option<Token<'a>> {
        unimplemented!()
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(source: &'a str) -> Tokens<'a> {
        Tokens {
            iter: source.chars().peekable(),
            line: 1,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace()?;

        let token_type = match self.iter.next()? {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '#' => TokenType::Pound,
            '!' if self.iter.next_if_eq(&'=').is_some() => TokenType::BangEqual,
            '!' => TokenType::Bang,
            '=' if self.iter.next_if_eq(&'=').is_some() => TokenType::EqualEqual,
            '=' => TokenType::Equal,
            '<' if self.iter.next_if_eq(&'=').is_some() => TokenType::LessEqual,
            '<' => TokenType::Less,
            '>' if self.iter.next_if_eq(&'=').is_some() => TokenType::GreaterEqual,
            '>' => TokenType::Greater,
            '"' => return self.string(),
            num if num >= '0' && num <= '9' => return self.number(),
            _ => TokenType::Error,
        };
        Some(Token {
            token_type,
            token: "",
            line: 0,
            start: 0,
        })
    }
}

pub fn compile(source: &str) {
    let tokens = Tokens::from(source);
    let mut prev_line = 0;

    for token in tokens {
        if token.line == prev_line {
            print!("{:>4} ", token.line);
        } else {
            print!("   | ");
        }
        println!("{:?} {}", token.token_type, token.token);
        prev_line = token.line;
    }
}
