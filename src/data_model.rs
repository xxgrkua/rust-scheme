use std::ptr::NonNull;
#[feature(get_mut_unchecked)]
use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    pub(crate) content: Link<'a>,
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Link<'a> {
    More(Rc<ExpressionContent<'a>>),
    Nil,
    Undefined,
}

impl<'a> Display for Link<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::More(expression) => write!(f, "{}", expression),
            Self::Nil => write!(f, "()"),
            Self::Undefined => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionContent<'a> {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(&'a str),
    PairLink(Pair<'a>),
    VectorLink(Vec<Link<'a>>),
    Promise(Promise<'a>),
}

impl<'a> Display for ExpressionContent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "\"{}\"", string),
            Self::Boolean(boolean) => {
                if *boolean {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::PairLink(pair) => write!(f, "{}", pair),
            Self::VectorLink(vector) => {
                write!(f, "#(")?;
                for (index, link) in vector.iter().enumerate() {
                    write!(f, "{}", link)?;
                    if index != vector.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
            Self::Promise(_) => write!(f, "#<promise>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pair<'a> {
    pub(crate) car: Link<'a>,
    pub(crate) cdr: Link<'a>,
}

impl<'a> Display for Pair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", &self.car)?;
        let mut cdr = &self.cdr;
        loop {
            if let Link::More(expression) = &cdr {
                if let ExpressionContent::PairLink(pair) = expression.as_ref() {
                    write!(f, " {}", pair.car)?;
                    cdr = &pair.cdr;
                } else {
                    write!(f, " . {}", cdr)?;
                    break;
                }
            } else {
                break;
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Promise<'a> {
    content: Link<'a>,
    frame: Frame<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame<'a> {
    content: FrameLink<'a>,
}

type FrameLink<'a> = NonNull<FrameNode<'a>>;

#[derive(Debug, Clone, PartialEq)]
struct FrameNode<'a> {
    data: HashMap<String, Expression<'a>>,
    parent: Option<FrameLink<'a>>,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        unsafe {
            Self {
                content: NonNull::new_unchecked(Box::into_raw(Box::new(FrameNode {
                    data: HashMap::new(),
                    parent: None,
                }))),
            }
        }
    }

    pub fn define(&mut self, name: &str, expression: Expression<'a>) {
        unsafe {
            (*self.content.as_ptr()).define(name, expression);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Expression<'a>> {
        unsafe { (*self.content.as_ptr()).lookup(name) }
    }
}

impl<'a> FrameNode<'a> {
    pub fn define(&mut self, name: &str, expression: Expression<'a>) {
        self.data.insert(name.to_string(), expression);
    }

    pub fn lookup(&self, name: &str) -> Option<&Expression<'a>> {
        unsafe {
            match self.data.get(name) {
                Some(expression) => Some(expression),
                None => match self.parent {
                    Some(frame) => (*frame.as_ptr()).lookup(name),
                    None => None,
                },
            }
        }
    }
}

enum Applicable<'a> {
    SpecialForm(SpecialForm),
    Procedure(Procedure<'a>),
}

enum SpecialForm {
    And,
    Begin,
    Case,
    Cond,
    Define,
    Delay,
    Do,
    If,
    Lambda,
    Let,
    LetRec,
    LetRecSyntax,
    LetStar,
    LetSyntax,
    Or,
    QuasiQuote,
    Quote,
    Set,
    SyntaxRules,
    Unquote,
    UnquoteSplicing,
}

enum Procedure<'a> {
    Builtin(Box<dyn BuiltinProcedure>),
    Lambda(LambdaProcedure<'a>),
}

trait BuiltinProcedure {}

struct LambdaProcedure<'a> {
    formals: Vec<String>,
    body: Expression<'a>,
    frame: Frame<'a>,
}
