use crate::token::{Token, Lexer};

pub struct Repl { }

impl Repl {
    pub fn new() -> Repl {
        return Repl {
        };
    }

    pub fn line(&self, line: &str) -> Vec<Token> {
        let lex = Lexer::new(line);
        let mut out = vec![];

        for token in lex.into_iter() {
            out.push(token);
        }

        return out;
    }

}
