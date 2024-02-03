use std::ops::Deref;
use std::ptr::NonNull;
use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::error::ApplyError;
use crate::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub(crate) content: Link,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Link {
    More(Rc<ExpressionContent>),
    Nil,
}

impl Link {
    pub fn as_ref(&self) -> Option<&Rc<ExpressionContent>> {
        match self {
            Self::More(expression) => Some(&expression),
            Self::Nil => None,
        }
    }
    pub fn as_deref(&self) -> Option<&ExpressionContent> {
        match self {
            Self::More(expression) => Some(expression.deref()),
            Self::Nil => None,
        }
    }

    pub fn outer_iter(&self) -> OuterIter<'_> {
        OuterIter {
            next: self.as_ref(),
        }
    }

    pub fn inner_iter(&self) -> InnerIter<'_> {
        InnerIter {
            next: self.as_deref(),
        }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::More(expression) => write!(f, "{}", expression),
            Self::Nil => write!(f, "()"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionContent {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(String),
    PairLink(Pair),
    VectorLink(Vec<Link>),
    Promise(Promise),
}

impl<'a> Display for ExpressionContent {
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
            Self::Promise(_) => write!(f, "#[promise]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pair {
    pub(crate) car: Link,
    pub(crate) cdr: Link,
}

impl Display for Pair {
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

pub(crate) struct OuterIter<'a> {
    next: Option<&'a Rc<ExpressionContent>>,
}

impl<'a> Iterator for OuterIter<'a> {
    type Item = &'a Rc<ExpressionContent>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.and_then(|content| match content.as_ref() {
            ExpressionContent::PairLink(pair) => {
                self.next = pair.cdr.as_ref();
                pair.car.as_ref()
            }
            _ => None,
        })
    }
}

pub(crate) struct InnerIter<'a> {
    next: Option<&'a ExpressionContent>,
}

impl<'a> Iterator for InnerIter<'a> {
    type Item = &'a ExpressionContent;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.and_then(|content| match content {
            ExpressionContent::PairLink(pair) => {
                self.next = pair.cdr.as_deref();
                pair.car.as_deref()
            }
            _ => None,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Promise {
    pub(crate) content: Link,
    pub(crate) frame: FrameLink,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Thunk {
    pub(crate) content: Link,
    pub(crate) frame: FrameLink,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub(crate) content: FrameLink,
}

pub(crate) type FrameLink = NonNull<FrameNode>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FrameNode {
    data: HashMap<String, Value>,
    parent: Option<FrameLink>,
}

impl Frame {
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

    pub fn new_with_parent(parent: &Self) -> Self {
        unsafe {
            Self {
                content: NonNull::new_unchecked(Box::into_raw(Box::new(FrameNode {
                    data: HashMap::new(),
                    parent: Some(parent.content),
                }))),
            }
        }
    }

    pub fn make_child(&self) -> Self {
        unsafe {
            Self {
                content: NonNull::new_unchecked(Box::into_raw(Box::new(FrameNode {
                    data: HashMap::new(),
                    parent: Some(self.content),
                }))),
            }
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        unsafe {
            (*self.content.as_ptr()).define(name, value);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        unsafe { (*self.content.as_ptr()).lookup(name) }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.content.as_ptr());
        }
    }
}

impl FrameNode {
    pub fn define(&mut self, name: &str, value: Value) {
        self.data.insert(name.to_string(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        unsafe {
            match self.data.get(name) {
                Some(value) => Some(value),
                None => match self.parent {
                    Some(frame) => (*frame.as_ptr()).lookup(name),
                    None => None,
                },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Procedure {
    Builtin(BuiltinProcedure),
    Lambda(LambdaProcedure),
}

impl Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builtin(builtin) => write!(f, "{}", builtin),
            Self::Lambda(lambda) => write!(f, "{}", lambda),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinProcedure {
    pub(crate) name: &'static str,
    pub(crate) function: fn(Vec<Value>) -> Result<Value, ApplyError>,
}

impl Display for BuiltinProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[{}]", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaProcedure {
    pub(crate) formals: Vec<String>,
    pub(crate) body: Link,
    pub(crate) frame: Frame,
}

impl Display for LambdaProcedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[lambda at {:p}]", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Expression(Expression),
    Procedure(Procedure),
    Thunk(Thunk),
    Unspecified,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expression) => write!(f, "{}", expression),
            Self::Procedure(procedure) => write!(f, "{}", procedure),
            Self::Thunk(_) => write!(f, "#[thunk]"),
            Self::Unspecified => write!(f, "#[unspecified]"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mem_size() {
        println!("size of Expression: {}", std::mem::size_of::<Expression>());
        println!("size of Link: {}", std::mem::size_of::<Link>());
        println!(
            "size of RcExpressionContent: {}",
            std::mem::size_of::<Rc<ExpressionContent>>()
        );
        println!(
            "size of ExpressionContent: {}",
            std::mem::size_of::<ExpressionContent>()
        );
        println!("size of Pair: {}", std::mem::size_of::<Pair>());
        println!("size of Promise: {}", std::mem::size_of::<Promise>());
        println!("size of Thunk: {}", std::mem::size_of::<Thunk>());
        println!("size of Frame: {}", std::mem::size_of::<Frame>());
        println!("size of FrameLink: {}", std::mem::size_of::<FrameLink>());
        println!("size of FrameNode: {}", std::mem::size_of::<FrameNode>());
        println!(
            "size of SpecialForm: {}",
            std::mem::size_of::<SpecialForm>()
        );
        println!("size of Procedure: {}", std::mem::size_of::<Procedure>());
        println!(
            "size of BuiltinProcedure: {}",
            std::mem::size_of::<BuiltinProcedure>()
        );
        println!(
            "size of LambdaProcedure: {}",
            std::mem::size_of::<LambdaProcedure>()
        );
        println!("size of Value: {}", std::mem::size_of::<Value>());
    }
}
