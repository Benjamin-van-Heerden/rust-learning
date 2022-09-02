fn main() {
    println!("Hello, world!");
    struct This {
        _my: String,
    }

    let _s = This {
        _my: String::from("s"),
    };
    println!("{}", String::from("hi"));
}
