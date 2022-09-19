use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

pub fn closures() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: i32| -> i32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_millis(500));
        num
    };

    println!("{}", expensive_closure(5));

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // the following two will not compile since the input types cannot be inferred here
    // when closures are used in general it will be the case that the parameters can be inferred
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;

    // capturing references or moving ownership?
    let mut list = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list); // this will not compile
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 11,
            height: 2,
        },
        Rectangle {
            width: 8,
            height: 15,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
