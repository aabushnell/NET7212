mod list;
mod stack;

use list::List::{self, Nil, Cons};
use stack::Stack;

fn main() {
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let mut i = l.iter();
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), None);
}
