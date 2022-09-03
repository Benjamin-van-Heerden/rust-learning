// pub use crate::{back_of_house as boh, front_of_house::hosting};

mod back_of_house;
mod front_of_house;

mod customer {
    use crate::{back_of_house as boh, front_of_house::hosting};
    pub fn eat_at_restaurant() {
        // can refer to front_of_house module since eat_at_restaurant and it are within the same file - they are siblings

        // absolute path
        hosting::add_to_waitlist();

        // relative path
        hosting::add_to_waitlist();

        let mut meal = boh::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");
        println!("I would like {} toast please.", meal.toast);

        let order1 = boh::Appetizer::Salad;
        let order2 = boh::Appetizer::Soup;
    }
}

#[cfg(test)]
mod run_tests {
    #[test]
    fn basic() {
        let x = 5;
        assert_eq!(x, 5);
    }
}
