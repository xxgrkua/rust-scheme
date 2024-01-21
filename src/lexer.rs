use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Identifier(&'a str),
    Boolean(&'a str),
    Number(&'a str),
    Character(&'a str),
    String(&'a str),
    OpenParenthesis,
    CloseParenthesis,
    VectorOpen,
    ByteVectorOpen,
    Quote,
    Backquote,
    Comma,
    CommaAt,
    Dot,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(string) => write!(f, "{}", string),
            Self::Boolean(string) => write!(f, "{}", string),
            Self::Number(string) => write!(f, "{}", string),
            Self::Character(string) => write!(f, "{}", string),
            Self::String(string) => write!(f, "{}", string),
            Self::OpenParenthesis => write!(f, "("),
            Self::CloseParenthesis => write!(f, ")"),
            Self::VectorOpen => write!(f, "#("),
            Self::ByteVectorOpen => write!(f, "#u8("),
            Self::Quote => write!(f, "'"),
            Self::Backquote => write!(f, "`"),
            Self::Comma => write!(f, ","),
            Self::CommaAt => write!(f, ",@"),
            Self::Dot => write!(f, "."),
        }
    }
}

fn read_identifier(src: &str, mut index: usize, length: usize) -> (Token, usize) {
    (Token::Dot, index)
}

pub fn tokenizer(src: &str) -> Vec<Token> {
    let token_list = vec![];
    let mut index = 0;
    let length = src.len();
    while index < length {
        match src.get(index..index + 1) {
            Some(string) => println!("{}", string),
            None => panic!(),
        }
        index += 1;
    }

    token_list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = "(define x 4)";
        println!("{:?}", tokenizer(src));
    }
}
