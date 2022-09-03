fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // associated function on enums
    println!("{}", home.p());
    println!("{}", loopback.p());

    let other_home = TypedIpAddr::V4(127, 0, 0, 1);

    let other_loopback = TypedIpAddr::V6(String::from("::1"));

    // Option is a very powerful enum that provides null safety
    // Options are used when a value could be something or nothing
    // rust does not have nulls - but we have a way of knowing whether a value is present or not (Option)

    let some_number = Some(5);
    let just_a_number = 6;
    let some_char = Some('s');
    let absent_number: Option<i32> = None;

    sum_if_there(&some_number, just_a_number);
    sum_if_there(&absent_number, just_a_number);
}

fn sum_if_there(v1: &Option<i32>, v2: i32) {
    match v1 {
        None => println!("The number is not defined"),
        Some(n) => println!("The sum of the two numbers is {}", n + v2),
    }
}

// enums are useful when we want to have data of a known form
// they can even store additional data
enum IpAddr {
    V4(String),
    V6(String),
}
// the name of each enum variant becomes like an associated function
// IpAddr::V4() is a function that takes a String argument and returns an IpAddr Type
// we automatically get this constructor
// each variant can have different types as well

enum TypedIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// you can put any type of data in an enum even structs or other enums

// we are also able to define methods on enums
impl IpAddr {
    fn p(&self) -> String {
        match self {
            IpAddr::V4(ad) => format!("V4 with address {}", ad),
            IpAddr::V6(ad) => format!("V6 with address {}", ad),
        }
    }
}
