#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello!")
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 1000 {
            panic!("Guess value should be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Caroline");
        assert!(
            result.contains("Caroline"),
            "Greeting did not contain name, value was instead `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

}