mod guess;
mod rectangle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = rectangle::Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = rectangle::Rectangle {
            width: 8,
            height: 8,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_greeting() {
        let greeting = String::from("Hello there General Kenobi");
        assert!(
            greeting.to_lowercase().contains("greeting"),
            "greeting did not contain 'greeting', value was `{}`",
            greeting
        );
    }

    #[test]
    #[should_panic(expected = "should be between 1 and 100")] // this is like a contains on the error message
    #[ignore] // if the test is expensive (takes long to run e.g.)
    fn test_that_should_panic() {
        guess::Guess::new_guess(101);
    }

    // tests also pass or fail on the result type
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
