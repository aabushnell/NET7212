use std::{fmt, mem};

#[derive(Debug)]
pub enum List {
    Nil,
    Cons(i32, Box<List>),
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Nil => write!(f, " "),
            List::Cons(val, next) => write!(f, " {val}{next}"),
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    head: List,
    size: usize,
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.head)
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            head: List::Nil,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, val: i32) {
        self.head = List::Cons(val, Box::new(mem::replace(&mut self.head, List::Nil)));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, List::Nil) {
            List::Nil => None,
            List::Cons(val, next) => {
                self.head = *next;
                self.size -= 1;
                Some(val)
            }
        }
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(37);
    s.push(99);
    s.push(12);

    println!("Stack: {s}");

    let first = s.pop().expect("stack is empty???");

    println!("Top of stack: {first}");

    println!("Stack: {s}");
}
