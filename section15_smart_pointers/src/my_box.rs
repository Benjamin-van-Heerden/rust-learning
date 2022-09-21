pub fn my_box() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);
    let w = MyBox::new(x);
    assert_eq!(5, *w);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    
}

fn hello(name: &str) {
    println!("hello {name}")
}

use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}




















