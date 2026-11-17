use std::fmt;
use std::mem;
use std::str::FromStr;

use List::{Nil, Cons};

#[derive(Debug, PartialEq)]
pub enum List {
  Nil,
  Cons(i32, Box<List>),
}

impl List {
  pub fn iter(&self) -> Iter<'_> {
    Iter { cur: self }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_> {
    IterMut { cur: Some(self) }
  }
}

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Nil => write!(f, " "),
      Cons(val, next) => write!(f, " {val}{next}"),
    }
  }
}

impl From<Vec<i32>> for List {
  fn from(mut v: Vec<i32>) -> Self {
    let mut l = Nil;
    while let Some(x) = v.pop() {
      l = Cons(x, Box::new(mem::replace(&mut l, Nil)));
    }
    l
  }
}

impl From<List> for Vec<i32> {
  fn from(l: List) -> Self {
    l.into_iter().collect()
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseListError;

impl FromStr for List {
  type Err = ParseListError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.split_whitespace()
      .map(|num| num.parse::<i32>())
      .collect::<Result<Vec<i32>, _>>()
      .map(List::from)
      .map_err(|_| ParseListError)
  }
}

pub struct Iter<'a> {
  cur: &'a List,
}

impl<'a> Iterator for Iter<'a>{
  type Item = &'a i32;

  fn next(&mut self) -> Option<Self::Item> {
    match self.cur {
      Nil => None,
      Cons(val, next) => {
        self.cur = next;
        Some(val)
      },
    }
  }
}

impl<'a> IntoIterator for &'a List {
  type Item = &'a i32;
  type IntoIter = Iter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    Iter { cur: self }
  }
}

pub struct IterMut<'a> {
  cur: Option<&'a mut List>,
}

impl<'a> Iterator for IterMut<'a> {
  type Item = &'a mut i32;

  fn next(&mut self) -> Option<Self::Item> {
    let cur = self.cur.take()?;
    match cur {
      Nil => None,
      Cons(val, next) => {
        self.cur = Some(next);
        Some(val)
      },
    }
  }
}

impl<'a> IntoIterator for &'a mut List {
  type Item = &'a mut i32;
  type IntoIter = IterMut<'a>;

  fn into_iter(self) -> Self::IntoIter {
    IterMut { cur: Some(self) }
  }
}

pub struct IntoIter {
  cur: List,
}

impl Iterator for IntoIter {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    match mem::replace(&mut self.cur, Nil) {
      Nil => None,
      Cons(val, next) => {
        self.cur = *next;
        Some(val)
      }
    }
  }
}

impl IntoIterator for List {
  type Item = i32;
  type IntoIter = IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter { cur: self }
  }
}
