use std::fmt;
use std::mem;
use std::str::FromStr;

use crate::list::List::{self, Cons, Nil};

#[derive(Debug, PartialEq)]
pub struct Stack {
  head: List,
  size: usize,
}

impl Stack {
  pub fn new() -> Self {
    Stack { head: Nil, size: 0 }
  }

  pub fn len(&self) -> usize {
    self.size
  }

  pub fn push(&mut self, val: i32) {
    self.head = Cons(val, Box::new(mem::replace(&mut self.head, Nil)));
    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Nil) {
      Nil => None,
      Cons(val, next) => {
        self.head = *next;
        self.size -= 1;
        Some(val)
      }
    }
  }
}

impl fmt::Display for Stack {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.head)
  }
}

impl From<Vec<i32>> for Stack {
  fn from(v: Vec<i32>) -> Stack {
    let size = v.len();
    Stack {
      head: List::from(v),
      size: size,
    }
  }
}

impl From<Stack> for Vec<i32> {
  fn from(s: Stack) -> Vec<i32> {
    Vec::<i32>::from(s.head)
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStackError;

impl FromStr for Stack {
  type Err = ParseStackError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.strip_prefix('[')
      .and_then(|s| s.strip_suffix(']'))
      .ok_or(ParseStackError)
      .and_then(|s| s.parse::<List>().map_err(|_| ParseStackError))
      .map(|list| Stack::from(Vec::from(list)))
  }
}
