pub fn rc() {
    // the Rc smart pointer is used when multiple ownership is required, e.g. with graph structures
    // it is essentially a counter that keeps track of the number of refs to a value are in scope
    // to determine if the value is still in use - if there are zero refs the value is dropped

    // consider as an example two cons lists that share a part of a third
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    let b = Cons(5, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    let c = Cons(6, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
}

use std::rc::Rc;

use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
