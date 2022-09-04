pub fn lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    {
        let string3 = String::from("xyz");
        let result1 = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result1);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// lifetime annotations also exist on structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}
