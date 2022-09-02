fn main() {
    // the issue with ownership every time is that we have to return results
    // if we want to preserve their values
    // suppose we want to calculate the length of a string
    let unowned_str = String::from("mine");
    let len = calculate_take_ownership(unowned_str);
    // println!("{unowned_str}") - can't do this since ownership has transferred

    let owned_str = String::from("something");
    let no_take_len = calc_without_own(&owned_str); // pass it here "by reference"
    println!("{owned_str}"); // now this is fine

    let mut s = String::from("i can change");
    borrow_and_mutate(&mut s);
    println!("{s}");

    // mutable references have one big restriction, when we have a mutable ref to a value, we
    // can have NO OTHER REFS to that value, cannot borrow as mutable more than once
    // e.g. the below is not allowed

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // this makes sense and allows for mutation in a controlled sense - no race conditions and such

    // as always, we can use curly brackets to create a new scope, allowing for
    // multiple mutable references, just not simultaneous ones:

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        borrow_and_mutate(r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    borrow_and_mutate(r2);
    println!("{s}");

    // rust enforces a similar rule for combining mutable and immutable references.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM, when we try to do something with this mutable ref we will get errors
    // println!("{}, {}, and {}", r1, r2, r3);

    // users of an immutable reference do not expect the value to suddenly change so this behaviour is not allowed

    // note!! a reference's scope starts from where it is introduced to the last time it is used.
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // the scopes do not overlap so this behaviour is safe

}

fn calculate_take_ownership(s: String) -> usize {
    s.len()
}

// this is how we may refer to a value without taking ownership of it
// We call the action of creating a reference "borrowing". As in real life, if a person owns something,
// you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
fn calc_without_own(s: &String) -> usize {
    s.len()
}
// we cannot modify something we are borrowing!
// references are immutable by default

// mutable references - notice the syntax
fn borrow_and_mutate(s: &mut String) {
    s.push_str(", see");
}
