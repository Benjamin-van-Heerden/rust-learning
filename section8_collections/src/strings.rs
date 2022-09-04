pub fn string_ops() {
    let mut string = String::new();
    let data = "initial contents";
    let s = data.to_string(); // can also do in one line
                              // or just
    let or_just = String::from("initial contents");

    // mutable strings can grow and shringk just like vectors
    // can use the + operator
    let mut mut_string = String::from("initial");
    mut_string += " more";
    println!("{}", mut_string);
    mut_string.push_str(" even more");
    println!("{}", mut_string);
    // updating a single char can be achieved with just push
    mut_string.push('!');
    println!("{}", mut_string);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // note that we cannot add two String values together - we can add a reference of a String to a String
    // but &s2 is &String (how??) -> compiler coercion, turns &s2 into &s2[..]
    // so the statement s1 + &s2 takes ownership of s1 which means that s1 is no longer valid in the scope
    // println!("{}", s1); - won't work

    // for more complex concatenation it is advisable to use the format! macro
    // additionally, the format! macro (and macros in general) do not take ownership,
    // so we will still have access to the strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1); // - this is fine

    // how do we access individual string elements?
    // rust does not recommend string slicing due to all sorts of issues that could crop up with utf-8
    // as an example, what if we have a string like
    let strange_string = "Здравствуйте";
    // println!("{}", &strange_string[0]); - this is not allowed
    // the nearest we could get is probably something like
    let s_all = s.chars();
    let second_strange = strange_string.chars().nth(1).unwrap();
    let fourth_strange = strange_string.as_bytes()[3];
    println!("{}", second_strange);
    println!("{}", fourth_strange);
    println!("{:?}", s_all);
    println!("{:?}", strange_string.chars());
}
