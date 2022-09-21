struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

pub fn cleanup() {
    let c = CustomSmartPointer {
        data: String::from("stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("a");
    drop(c); // this is not really advisable - rust will do this automatically when c goes out of scope.
    println!("b")
}
