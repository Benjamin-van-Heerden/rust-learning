fn main() {
    let my_coin = Coin::Quarter(State::Texas);

    let value = value_in_cents(my_coin);

    let mut some_num = Some(3);
    let mut no_num: Option<i32> = None;

    some_num = plus_one_if_there(some_num);
    some_num = plus_one_if_there(some_num);
    match some_num {
        None => (), // this is shothand for nothing happens -> unit tuple
        Some(n) => println!("Value is now {}", n),
    }
    no_num = plus_one_if_there(no_num);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

enum State {
    New_York,
    Texas,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("I am a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => match state {
            State::New_York => {
                println!("Quarter from NY");
                25
            }
            State::Texas => {
                println!("Quarter from Texas");
                25
            }
        },
    }
}

fn plus_one_if_there(mut i: Option<i32>) -> Option<i32> {
    match i {
        Some(n) => {
            println!("Num is now {}", n + 1);
            Some(n + 1)
        }
        None => {
            println!("No number");
            None
        }
    }
}
