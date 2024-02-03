use std::rc::Rc;

use crate::{
    data_model::{Expression, ExpressionContent, Link, Pair},
    error::ParseError,
    lexer::{Token, TokenBuffer},
    number::Number,
};

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression> {
    if buffer.is_empty() {
        return Err(ParseError::EOF);
    }
    match *buffer.pop() {
        Token::Identifier(identifier) => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::Symbol(identifier.to_string()))),
        }),
        Token::Boolean(value) => match value {
            "#t" | "#true" => Ok(Expression {
                content: Link::More(Rc::new(ExpressionContent::Boolean(true))),
            }),
            "#f" | "#false" => Ok(Expression {
                content: Link::More(Rc::new(ExpressionContent::Boolean(false))),
            }),
            _ => unreachable!(),
        },
        Token::Number(number) => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::Number(Number::try_from(
                number,
            )?))),
        }),
        Token::String(string) => {
            let mut result = String::new();
            let mut start_escape = false;
            let mut start_hex = false;
            let mut hex_buffer = String::new();
            for character in string.chars() {
                match character {
                    '\\' => {
                        if start_escape {
                            result.push('\\');
                            start_escape = false;
                        } else {
                            start_escape = true;
                        }
                    }
                    '"' => {
                        if start_escape {
                            result.push('"');
                            start_escape = false;
                        } else {
                            // remove the first and last quote
                            continue;
                        }
                    }
                    'n' => {
                        if start_escape {
                            result.push('\n');
                            start_escape = false;
                        } else {
                            result.push('n');
                        }
                    }
                    'r' => {
                        if start_escape {
                            result.push('\r');
                            start_escape = false;
                        } else {
                            result.push('r');
                        }
                    }
                    't' => {
                        if start_escape {
                            result.push('\t');
                            start_escape = false;
                        } else {
                            result.push('t');
                        }
                    }
                    'b' => {
                        if start_escape {
                            result.push('\x08');
                            start_escape = false;
                        } else {
                            result.push('b');
                        }
                    }
                    'a' => {
                        if start_escape {
                            result.push('\x07');
                            start_escape = false;
                        } else {
                            result.push('a');
                        }
                    }
                    'x' => {
                        if start_escape {
                            start_hex = true;
                        } else {
                            result.push('x');
                        }
                    }
                    ';' => {
                        if start_hex {
                            start_hex = false;
                            start_escape = false;
                            let hex = u32::from_str_radix(&hex_buffer, 16).map_err(|_| {
                                ParseError::InvalidCharacterEscape(format!("\\x{}", hex_buffer))
                            })?;
                            result.push(char::from_u32(hex).ok_or(
                                ParseError::InvalidCharacterEscape(format!("\\x{}", hex_buffer)),
                            )?);
                            hex_buffer.clear();
                        } else {
                            result.push(';');
                        }
                    }
                    c => {
                        if start_hex {
                            if c.is_digit(16) {
                                hex_buffer.push(c);
                            } else {
                                return Err(ParseError::InvalidCharacterEscape(format!(
                                    "\\x{}{}",
                                    hex_buffer, c
                                )));
                            }
                        } else if start_escape {
                            return Err(ParseError::InvalidCharacterEscape(format!("\\{}", c)));
                        } else {
                            result.push(c);
                        }
                    }
                }
            }
            Ok(Expression {
                content: Link::More(Rc::new(ExpressionContent::String(result))),
            })
        }
        Token::Comment(_) => parse(buffer),
        Token::OpenParenthesis => parse_pair(buffer),
        Token::CloseParenthesis => Err(ParseError::MissingOpenParenthesis),
        Token::VectorOpen => {
            // vector is self-evaluating in R7RS
            let vector = vec![];
            parse_vector(buffer, vector)
        }
        Token::ByteVectorOpen => unimplemented!(),
        Token::Quote => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link::More(Rc::new(ExpressionContent::Symbol("quote".to_string()))),
                cdr: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link::Nil,
                }))),
            }))),
        }),
        Token::BackQuote => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link::More(Rc::new(ExpressionContent::Symbol("quasiquote".to_string()))),
                cdr: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link::Nil,
                }))),
            }))),
        }),
        Token::Comma => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link::More(Rc::new(ExpressionContent::Symbol("unquote".to_string()))),
                cdr: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link::Nil,
                }))),
            }))),
        }),
        Token::CommaAt => Ok(Expression {
            content: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link::More(Rc::new(ExpressionContent::Symbol(
                    "unquote-splicing".to_string(),
                ))),
                cdr: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link::Nil,
                }))),
            }))),
        }),
        Token::Dot => Err(ParseError::InvalidDot),
    }
}

fn parse_pair<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression> {
    if buffer.is_empty() {
        Err(ParseError::MissingCLoseParenthesis)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => {
                buffer.pop();
                Ok(Expression { content: Link::Nil })
            }
            Token::Dot => {
                buffer.pop();
                let rest = parse(buffer)?;
                if buffer.is_empty() {
                    Err(ParseError::MissingCLoseParenthesis)
                } else {
                    match *buffer.pop() {
                        Token::CloseParenthesis => Ok(rest),
                        _ => Err(ParseError::TooMoreObjects),
                    }
                }
            }
            _ => {
                let first = parse(buffer)?;
                let rest = parse_pair(buffer)?;
                Ok(Expression {
                    content: Link::More(Rc::new(ExpressionContent::PairLink(Pair {
                        car: first.content,
                        cdr: rest.content,
                    }))),
                })
            }
        }
    }
}

fn parse_vector<'a>(buffer: &mut TokenBuffer<'a>, mut vector: Vec<Link>) -> Result<Expression> {
    loop {
        if buffer.is_empty() {
            return Err(ParseError::MissingCLoseParenthesis);
        } else {
            match *buffer.peek() {
                Token::CloseParenthesis => {
                    buffer.pop();
                    return Ok(Expression {
                        content: Link::More(Rc::new(ExpressionContent::VectorLink(vector))),
                    });
                }
                _ => {
                    let first = parse(buffer)?;
                    vector.push(first.content);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize;

    use super::*;

    #[test]
    fn test_parse() {
        let mut buffer = tokenize("(#t #t #t)").unwrap();
        let expression = parse(&mut buffer).unwrap();
        // assert_eq!(expression.car().car().car().content, Some(1));
        println!("{:?}", expression);
        let mut buffer = tokenize("(1 2 3)").unwrap();
        println!("{:?}", buffer);
        println!("{:?}", parse(&mut buffer).unwrap());
        let mut buffer = tokenize("(2.3@4 2.3+5i +i)").unwrap();
        println!("{:?}", buffer);
        println!("{:?}", parse(&mut buffer).unwrap());
        println!("quote parse: {:?}", parse(&mut tokenize("'x").unwrap()));
        println!("parse empty: {:?}", parse(&mut tokenize("").unwrap()));
        println!(
            "string parse: {:?}",
            parse(&mut tokenize(r#""sdfsdf sdf\n sdfsd sdf \x03B1; \" asd""#).unwrap()).unwrap()
        );
        println!(
            "list parse: {}",
            parse(&mut tokenize("(1 2 3)").unwrap()).unwrap()
        );
        println!(
            "vector parse: {}",
            parse(&mut tokenize("#(1 2 3)").unwrap()).unwrap()
        );
        println!("singleton tokenize {:?}", tokenize("(+)"));
        println!("singleton parse {:?}", parse(&mut tokenize("(+)").unwrap()))
    }

    #[test]
    fn test_dotted_pair() {
        assert_eq!(
            parse(&mut tokenize("(1 . 2 3)").unwrap()),
            Err(ParseError::TooMoreObjects)
        );
        assert_eq!(
            parse(&mut tokenize("(1 2 3").unwrap()),
            Err(ParseError::MissingCLoseParenthesis)
        );
        // assert_eq!(
        //     parse(&mut tokenize("(1 2 3))").unwrap()),
        //     Err(ParseError::MissingOpenParenthesis)
        // );
        assert_eq!(
            parse(&mut tokenize("(1 2 3 4 . 5)").unwrap()),
            parse(&mut tokenize("(1 . (2 . (3 . (4 . 5))))").unwrap())
        );
        assert_eq!(
            parse(&mut tokenize("(a b c d e)").unwrap()),
            parse(&mut tokenize("(a . (b . (c . (d . (e . ())))))").unwrap())
        );
        println!(
            "dotted pair parse: {}",
            parse(&mut tokenize("(1 2 3 4 . 5)").unwrap()).unwrap()
        );
    }

    #[test]
    fn test() {
        println!(
            "test nested: {}",
            parse(&mut tokenize("(1 2 3 #(4 5 6 (7 8 . 9)))").unwrap()).unwrap()
        )
    }
}
