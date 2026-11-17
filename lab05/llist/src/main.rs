use List::{Cons, Nil};
use std::str::FromStr;
use std::{fmt, mem};

#[derive(Debug, PartialEq)]
pub enum List {
    Nil,
    Cons(i32, Box<List>),
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
    fn from(mut v: Vec<i32>) -> List {
        let mut l = Nil;
        while let Some(x) = v.pop() {
            l = Cons(x, Box::new(mem::replace(&mut l, Nil)));
        }
        l
    }
}

impl From<List> for Vec<i32> {
    fn from(mut l: List) -> Vec<i32> {
        let mut v: Vec<i32> = vec![];
        while let Cons(x, next) = mem::replace(&mut l, Nil) {
            v.push(x);
            l = *next;
        }
        v
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

pub struct ListIterator<'a> {
    cur: &'a List,
}

impl List {
    pub fn iter(&self) -> ListIterator {
        ListIterator { cur: self }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Nil => None,
            Cons(x, next) => Some(*x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Stack {
    head: List,
    size: usize,
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

fn main() {
    let mut v = vec![1, 2, 3];
    assert_eq!(
        List::from(v),
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    );

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(vec![1, 2, 3], Vec::<i32>::from(l));

    v = vec![1, 2, 3];
    assert_eq!(
        Stack::from(v),
        Stack {
            head: Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))),
            size: 3,
        }
    );

    let s = Stack {
        head: Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))),
        size: 3,
    };
    assert_eq!(vec![1, 2, 3], Vec::<i32>::from(s));

    let mut s1 = Stack::new();
    s1.push(10);
    s1.push(20);
    s1.push(30);

    let s2: Stack = "[ 30 20 10 ]".parse().unwrap();

    assert_eq!(s1, s2);
}
