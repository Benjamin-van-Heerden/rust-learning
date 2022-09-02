use std::io;

fn main() {
    fizz_buzz();
    ///////////////////////////
    // variables and mutability
    ///////////////////////////
    let x = 5;
    println!("The value of x is {x}");

    // not allowed due to immutability
    // x = 6;
    // println!("The value of x is {x}");

    // variables can be redefined - this is shadowing
    let x = 6;
    println!("The value of x is {x}");

    // mutability must be explicitly defined
    let mut y = 6;

    y = 8;
    println!("The value of y is {y}");

    // constants are immutable anb must be type annotated
    // they can be declared in any scope, even global which makes them useful
    // constants may be st only to a constant expression and not something that
    // is calculated at runtime
    const SECONDS_IN_HOUR: i32 = 60;
    println!("There are {SECONDS_IN_HOUR} seconds in an hour");

    // shadowing, as declared above, the most recent declaration of a variable "overshadows" all others
    // this can be useful in different scopes
    {
        let x = x * 2;
        println!("x times 2 is {x}")
    }
    println!("The value of x is still {x}");

    // why is this useful?
    // simple example
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");

    // this behaviour would not work with mut since the type would change
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("There are {spaces} spaces")

    /////////////
    // data types
    /////////////

    // rust must know the types of variables at compile time
    // in cases where many types are possible we need to explicitly tell the compiler the type
    let integer: u32 = "42".parse().expect("Not a number");
    let float = 42.2f64; // numbers can be typed with a suffix

    // numeric operations work as would be expected
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0 - for float division both numbers need to be of type float

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation
                         // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructured assignment is also possible
    let (x, y, z) = tup;

    // accessing tuple elements is achieved with .index
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays - must have same type, in rust arrays are of fixed length
    let a = [1, 2, 3, 4, 5];
    // vectors - which will be introduced later can shrink and grow in size
    // vectors are almost always more convenient unless the size of the array is known
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5]; // same as [3, 3, 3, 3, 3]

    // array accession is by index in []
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // accessing an array element that does not exist at runtime will cause a panic!
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // error handling will be discussed later

    another_function();
    with_params(5);
    let a = expressions_demo(5);
    println!("{a}")
}

////////////
// functions
////////////

fn another_function() {
    println!("From elsewhere");
}

fn with_params(x: i32) {
    println!("The number is {x}")
}

// rust is expression based so that we can do things like this
// the value that the function assumes is that of the last expression
// no explicit return is required though can be used if we want to return early
fn expressions_demo(x: i32) -> i32 {
    let y = {
        let z = x + 2;
        z
    };
    y
}
// expressions are wrapped in blocks {}

fn plus_one(x: i32) -> i32 {
    x + 1 //; placing this semicolon turns the last expression into an assignment which does
          // not return the value of the assignment - in other languages this might work
}

///////////////
// control flow
///////////////

fn control_flow() {
    // ifs
    let number = 3;

    // don't need and else
    if number < 5 {
        println!("something");
    } else if number == 2 {
        println!("something else if");
    } else {
        println!("something else");
    }

    let condition = true;
    // can use them inline
    let x = if condition { 5 } else { 2 };
    // rust won't allow different return types for the above statement

    // loops
    // infinite loop
    loop {
        println!("do something infinitely")
    }

    // loops execute until they are ordered to stop
    // loops support break and continue and they behave as would be expected
    // break may be succeeded with a return-like statement
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // assignment
        }
    };

    // while loops take conditionals
    while count < 100 {
        count += 1;
    }

    // loop through collections with for
    let col = [(1, "String", 6.4), (2, "Another String", 12.8)];
    // this is probably the most useful loop construct
    for (a, b, c) in col {
        println!("{a}, {b}, {c}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}

fn fizz_buzz() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        }
    }
}
