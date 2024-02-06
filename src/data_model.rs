use std::ops::Deref;
use std::ptr::NonNull;
use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::builtin;
use crate::error::ApplyError;
use crate::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub(crate) content: Link,
}

impl Expression {
    pub fn is_number(&self) -> bool {
        self.content.is_number()
    }

    pub fn is_symbol(&self) -> bool {
        self.content.is_symbol()
    }

    pub fn is_string(&self) -> bool {
        self.content.is_string()
    }

    pub fn is_boolean(&self) -> bool {
        self.content.is_boolean()
    }

    pub fn is_pair(&self) -> bool {
        self.content.is_pair()
    }

    pub fn is_vector(&self) -> bool {
        self.content.is_vector()
    }

    pub fn is_promise(&self) -> bool {
        self.content.is_promise()
    }

    pub(crate) fn as_link(&self) -> &Link {
        &self.content
    }

    pub(crate) fn as_content(&self) -> Option<&ExpressionContent> {
        self.content.as_deref()
    }

    pub fn as_number(&self) -> Option<&Number> {
        self.content.as_number()
    }

    pub fn as_string(&self) -> Option<&str> {
        self.content.as_string()
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        self.content.as_boolean()
    }

    pub fn as_symbol(&self) -> Option<&str> {
        self.content.as_symbol()
    }

    pub(crate) fn as_pair(&self) -> Option<&Pair> {
        self.content.as_pair()
    }

    pub(crate) fn as_vector(&self) -> Option<&Vec<Link>> {
        self.content.as_vector()
    }

    pub(crate) fn as_promise(&self) -> Option<&Promise> {
        self.content.as_promise()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl From<bool> for Expression {
    fn from(boolean: bool) -> Self {
        Self {
            content: Link::from(boolean),
        }
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

    pub fn iter(&self) -> Iter<'_> {
        Iter { next: Some(&self) }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_number(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::Number(_)))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::Symbol(_)))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::String(_)))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::Boolean(_)))
    }

    pub fn is_pair(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::PairLink(_)))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::VectorLink(_)))
    }

    pub fn is_promise(&self) -> bool {
        matches!(self.as_deref(), Some(ExpressionContent::Promise(_)))
    }

    pub fn as_number(&self) -> Option<&Number> {
        self.as_deref()?.as_number()
    }

    pub fn as_string(&self) -> Option<&str> {
        self.as_deref()?.as_string()
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        self.as_deref()?.as_boolean()
    }

    pub fn as_symbol(&self) -> Option<&str> {
        self.as_deref()?.as_symbol()
    }

    pub fn as_pair(&self) -> Option<&Pair> {
        self.as_deref()?.as_pair()
    }

    pub fn as_vector(&self) -> Option<&Vec<Link>> {
        self.as_deref()?.as_vector()
    }

    pub fn as_promise(&self) -> Option<&Promise> {
        self.as_deref()?.as_promise()
    }
}

impl From<bool> for Link {
    fn from(boolean: bool) -> Self {
        Self::More(Rc::new(ExpressionContent::from(boolean)))
    }
}

impl From<Link> for Expression {
    fn from(link: Link) -> Self {
        Self { content: link }
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

impl ExpressionContent {
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Self::Number(number) => Some(number),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    pub fn as_pair(&self) -> Option<&Pair> {
        match self {
            Self::PairLink(pair) => Some(pair),
            _ => None,
        }
    }

    pub fn as_vector(&self) -> Option<&Vec<Link>> {
        match self {
            Self::VectorLink(vector) => Some(vector),
            _ => None,
        }
    }

    pub fn as_promise(&self) -> Option<&Promise> {
        match self {
            Self::Promise(promise) => Some(promise),
            _ => None,
        }
    }
}

impl From<bool> for ExpressionContent {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
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

impl Pair {
    pub fn car(&self) -> Link {
        return self.car.clone();
    }

    pub fn cdr(&self) -> Link {
        return self.cdr.clone();
    }
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

pub(crate) struct Iter<'a> {
    next: Option<&'a Link>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Link;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.and_then(|content| match content.as_deref() {
            Some(ExpressionContent::PairLink(pair)) => {
                self.next = Some(&pair.cdr);
                Some(&pair.car)
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
pub struct Thunk {
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
pub(crate) enum SpecialForm {
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

impl From<BuiltinProcedure> for Procedure {
    fn from(builtin: BuiltinProcedure) -> Self {
        Self::Builtin(builtin)
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

impl From<LambdaProcedure> for Procedure {
    fn from(lambda: LambdaProcedure) -> Self {
        Self::Lambda(lambda)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Expression(Expression),
    Procedure(Procedure),
    Thunk(Thunk),
    Unspecified,
}

impl Value {
    pub fn as_expression(&self) -> Option<&Expression> {
        match self {
            Self::Expression(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn as_procedure(&self) -> Option<&Procedure> {
        match self {
            Self::Procedure(procedure) => Some(procedure),
            _ => None,
        }
    }

    pub fn as_thunk(&self) -> Option<&Thunk> {
        match self {
            Self::Thunk(thunk) => Some(thunk),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Self::Expression(expression) => expression.as_number(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::Expression(expression) => expression.as_string(),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Expression(expression) => expression.as_boolean(),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Expression(expression) => expression.as_symbol(),
            _ => None,
        }
    }

    pub(crate) fn as_pair(&self) -> Option<&Pair> {
        match self {
            Self::Expression(expression) => expression.as_pair(),
            _ => None,
        }
    }

    pub(crate) fn as_vector(&self) -> Option<&Vec<Link>> {
        match self {
            Self::Expression(expression) => expression.as_vector(),
            _ => None,
        }
    }

    pub(crate) fn as_promise(&self) -> Option<&Promise> {
        match self {
            Self::Expression(expression) => expression.as_promise(),
            _ => None,
        }
    }
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

impl From<Expression> for Value {
    fn from(expression: Expression) -> Self {
        Self::Expression(expression)
    }
}

impl From<Link> for Value {
    fn from(link: Link) -> Self {
        Self::Expression(Expression::from(link))
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::from(Expression::from(boolean))
    }
}

impl From<Procedure> for Value {
    fn from(procedure: Procedure) -> Self {
        Self::Procedure(procedure)
    }
}

impl From<BuiltinProcedure> for Value {
    fn from(builtin: BuiltinProcedure) -> Self {
        Self::from(Procedure::from(builtin))
    }
}

impl From<LambdaProcedure> for Value {
    fn from(lambda: LambdaProcedure) -> Self {
        Self::from(Procedure::from(lambda))
    }
}

impl From<Thunk> for Value {
    fn from(thunk: Thunk) -> Self {
        Self::Thunk(thunk)
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
