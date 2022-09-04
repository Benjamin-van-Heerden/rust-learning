pub fn vec_ops() {
    let v2: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    v3.push(5);
    println!("{:?}", v3);

    // use borrowing here otherwise we lose access to v3
    let third = &v3[2]; // this is an immutable reference

    // remember that we cannot have mutable and immutable refs in the same scope
    // v3.push(6);
    // // the error in compilation only occurs when we want to refer to third again
    // // this is because it might no longer be defined.
    // // changing third to not be a reference will solve the problem - this may not be what we want
    // println!("The third element is {}", third);

    println!("{:?}", v3);
    let sixth = v3.get(5);
    match sixth {
        Some(i) => println!("The value is {}", i),
        None => println!("Not defined"),
    }

    // iteration over vectors is also convenient
    // notice that we take a slice of v3 (could as well have used v3[..])
    for i in &mut v3 {
        *i += 50; // this is the dereferencing operation
                  // - meaning that the loop takes a kind of ownership and can therefore mutate
    }
    println!("{:?}", v3);

    // what if we want more complex types in vectors?

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    print_row(row);
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn print_row(r: Vec<SpreadsheetCell>) {
    print!("[ ");
    for item in &r {
        match item {
            SpreadsheetCell::Int(i) => print!(" {} ", i),
            SpreadsheetCell::Float(f) => print!(" {} ", f),
            SpreadsheetCell::Text(t) => print!(" {} ", t),
        }
    }
    print!("]\n");
}
