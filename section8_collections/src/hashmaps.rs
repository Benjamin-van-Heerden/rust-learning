use std::collections::HashMap;

pub fn hashmap_ops() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // hashmaps store data on the heap - keys and values must be strictly typed
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    println!("{:?}", scores);

    // getting values from maps uses the get method
    let blue_team_name = String::from("Blue");
    let blue_score = scores.get(&blue_team_name).unwrap(); // notice it returs an option since the key may not exist
                                                           // note that unwrap will panick if the value is None - safer to use match - for this it is fine since
                                                           // we know it will not be None

    // this unwrap behaviour can be safe when we provide a default
    let red_score = scores.get("Red").unwrap_or(&0);
    let yellow_score = scores.get("Yellow").unwrap_or(&0);
    println!("{} {} {}", blue_score, red_score, yellow_score);

    // ownership works with types that implement the Copy trait, like i32 and &str
    // values on the head inserted into a hashmap become owned by the hashmap
    let a = String::from("a");
    let b = 32;
    scores.insert(a, b);
    println!("{}", b); // - this works fine since i32 implements Copy
                       // println!("{}", a); // this will not compile since scores is now the owner of a

    // updating a value associated with a key amounts to an insert - the newest value is assumed
    scores.insert("Blue".to_string(), 666);
    println!("{:?}", scores);

    // can also add a key value only if a key is not present
    scores.entry(String::from("Red")).or_insert(10); // since Red is already present this will do nothing
    println!("{:?}", scores);

    // updating a value based on an old value
    let text = "in in in sooth I know not why I am so sad";
    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0); // or insert returns a mutable reference
        *count += 1;
    }
    println!("{:?}", word_count);
    // hashmaps are not ordered

    
}
