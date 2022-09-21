#[derive(Debug, Clone)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn add_elem(&mut self, e: i32) {
        *self = Cons(e, Box::new(self.clone()));
    }
}

use List::{Cons, Nil};

pub fn box_heap() {
    let b = Box::new(5);
    println!("{}", b);

    let mut list = Cons(32, Box::new(Cons(42, Box::new(Cons(52, Box::new(Nil))))));
    list.add_elem(4);
    list.add_elem(777);
    println!("{:?}", list);
}
