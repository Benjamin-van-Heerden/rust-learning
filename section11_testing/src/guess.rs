pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new_guess(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value should be between 1 and 100 inclusive");
        }
        Guess { value }
    }
}
