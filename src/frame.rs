use std::{collections::HashMap, rc::Rc};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Frame {
    parent: Option<Rc<Frame>>,
    data: HashMap<String, Value>,
}

impl Frame {
    pub fn new(parent: &Option<Rc<Frame>>) -> Self {
        Frame {
            data: HashMap::new(),
            parent: parent.as_ref().map(|frame| Rc::clone(frame)),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.data.insert(name.to_string(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        match self.data.get(name) {
            Some(value) => Some(value),
            None => match &self.parent {
                Some(frame) => frame.lookup(name),
                None => None,
            },
        }
    }
}
