#[derive(Debug, PartialEq)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn search<'a>(contents: &'a str, search_term: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(search_term))
        .collect()
}

pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum(); // this will take ownership!
    println!("{}", total);

    let v2 = vec![5, 6, 7];
    let mapped = v2.iter().map(|x| x + 1); // this does not take ownership
                                           // notice that mapped returns a map object, not a vector - this is because the closure above
                                           // doesn't actually do anything since it is lazy - to get something we need to collect it
    let collected: Vec<_> = mapped.collect();
    println!("{:?}", collected);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:#?}", in_my_size);

    let lines = "this is lots of text\nto emulate\nsomething one might\nfind in a textfile\n or something\nis another";
    let term = "is";

    let lines_with_term = search(lines, term);
    println!("{:?}", lines_with_term);
}
