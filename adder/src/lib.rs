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
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than 1, got {}.", 
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}", 
                value
            );
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 8
        };

        let smaller = Rectangle {
            width: 4,
            height: 6
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 7,
            height: 8
        };

        let smaller = Rectangle {
            width: 4,
            height: 6
        };

        assert!(!smaller.can_hold(&larger)); 
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    pub fn greetings(name: &str) -> String {
        //format!("Hello {}!", name)
        String::from("Hello")
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greetings("Bob");
        assert!(
            result.contains("Bob"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}