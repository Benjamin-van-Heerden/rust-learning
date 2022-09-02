fn main() {
    let mut r1 = Rectangle {
        width: 10,
        height: 11,
    };

    let r2 = Rectangle {
        width: 8,
        height: 8,
    };

    r1.change_height(12);

    println!("The area is {}", r1.area());
    println!("I still have ownership of {:#?}", r1);
    let r1_holds_r2 = if r1.can_hold(&r2) { "can" } else { "cannot" };
    println!("r1 {r1_holds_r2} hold r2 within it");
    let sq = Rectangle::square(10);
    println!("I am a square:\n {:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation - like methods on classes
impl Rectangle {
    // must have self as first argument, notice it is just a reference to self, so that a method
    // on a struct defined this way cannot change the struct
    // methods can also borrow self mutably
    // here we don't want to take ownership, simply read the values
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // would we have wanted to change the value we use &mut self
    fn change_height(&mut self, new_height: u32) {
        self.height = new_height;
    }
}

// implementations can be split up and end up attached as methods to their structs
// function in an impl block are called associated functions
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// we can define "associated" methods that don't have self as their first param
// to call an associated function we use the :: operator
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
