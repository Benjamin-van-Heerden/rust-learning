fn main() {
    let literal_string = "hello";
    // literal_string.push_str // doesn't work
    let mut string = String::from("hello"); // this type of string can be mutated
    string.push_str(", world");
    println!("{string}");

    // why can String be mutated but literal cannot?
    // answer lies in memory and allocation

    // memory is automatically returned when a variable goes out of scope
    {
        let inner_string = String::from("inner");
    } // the scope is now over so inner_string is no longer valid

    // this
    {
        let x = 5;
        let y = x; // here x is copied and both x and y have value 5 - they are hard coded and land on the stack
    }
    // is different to this
    {
        let s1 = String::from("hello");
        // we don't know how large a String may be so it has memory
        // allocated on and lives on the heap
        let s2 = s1; // here both s1 and s2 have the same pointer to the heap

        // println!("{s1}"); // this is not allowed
        // for memory safety, after s2 is declared to "be" s1, s1 is no longer considered to be valid
        // we say s1 was "moved" into to s2

        println!("{s2}");

        // this solves the memory management problem, with only s2 valid, when it goes out of scope it alone will
        // free the memory and we are done!
    }

    // what if we want both of the values
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // here the data on the heap does get copied
    }

    let s1 = String::from("main owns this");
    let s2 = "I am just copied";

    take_ownership(s1);

    // println!("{s1}"); - this won't work since the function has taken ownership
    // s1 is no longer valid in this scope.

    make_copy(s2);

    println!("{s2}");

    let s3 = String::from("mine");
    let s4 = takes_and_gives_back(s3); // this is obviously fine
}

// ownership and functions

fn take_ownership(s: String) {
    println!("Now I have ownership, no longer '{s}'")
}

fn make_copy(s: &str) {
    println!("This can just happen with {s}")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// it can be annoying that anything we pass has to be returned back, but rust has 
// a mechanism called "references" that allows for use of a value without transferring ownership 