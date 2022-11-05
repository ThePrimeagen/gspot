use collection_macros::hashset;
use std::{collections::HashSet, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {

    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,


    Illegal,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirlybrace,
    Rsquirlybrace,
    Minus,

    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    Identifier(String),
    Int(usize),
}

static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "true" => Token::True,
    "false" => Token::False,
    "fn" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
};

#[derive(Debug)]
struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        loop {
            match self.read_char() {
                Some('*') => return Some(Token::Asterisk),
                Some('!') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::NotEqual);
                        }
                    }
                    return Some(Token::Bang);
                }
                Some('/') => return Some(Token::Slash),
                Some('>') => return Some(Token::Gt),
                Some('<') => return Some(Token::Lt),
                Some('-') => return Some(Token::Minus),
                Some('+') => return Some(Token::Plus),
                Some(',') => return Some(Token::Comma),
                Some('=') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Equal);
                        }
                    }
                    return Some(Token::Assign);
                }
                Some(';') => return Some(Token::Semicolon),
                Some('(') => return Some(Token::Lparen),
                Some(')') => return Some(Token::Rparen),
                Some('{') => return Some(Token::Lsquirlybrace),
                Some('}') => return Some(Token::Rsquirlybrace),

                Some(c) if c.is_digit(10) => {
                    let str = self.keep_reading(c, |c| c.is_digit(10));
                    let str = str.into_iter().collect::<String>();
                    return Some(Token::Int(
                        str::parse::<usize>(&str).expect("this should always work"),
                    ));
                }

                Some(c) if c.is_ascii_alphabetic() => {
                    let ident = self.keep_reading(c, |c| c.is_ascii_alphabetic());
                    let ident = ident.into_iter().collect::<String>();

                    if let Some((_, v)) = KEYWORDS.get_entry(&ident) {
                        return Some(v.clone());

                    }
                    return Some(Token::Identifier(ident));
                }

                Some(_) => return Some(Token::Illegal),
                _ => return None,
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Lexer<'a> {
        return Lexer {
            chars: code.chars().peekable(),
        };
    }

    fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }

    fn read_char(&mut self) -> Option<char> {
        return self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}
    }

    fn keep_reading(&mut self, c: char, f: impl Fn(&char) -> bool) -> Vec<char> {
        let mut out = vec![c];
        while let Some(c) = self.chars.next_if(&f) {
            out.push(c);
        }

        return out;
    }
}

#[cfg(test)]
mod test {

    use super::{Lexer, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lexer_iterator() {
        let input = "=+(){},;";
        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lsquirlybrace,
            Token::Rsquirlybrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input);

        assert_eq!(lexer.into_iter().collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_lexer_2() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);";
        let expected = vec![
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::Rparen,
            Token::Lsquirlybrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::Semicolon,
            Token::Rsquirlybrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::Lparen,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input);
        assert_eq!(lexer.into_iter().collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_lexer_3() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;";

        let expected = vec![
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::Rparen,
            Token::Lsquirlybrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::Semicolon,
            Token::Rsquirlybrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::Lparen,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lsquirlybrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rsquirlybrace,
            Token::Else,
            Token::Lsquirlybrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rsquirlybrace,
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input);
        assert_eq!(lexer.into_iter().collect::<Vec<Token>>(), expected);
    }
}
