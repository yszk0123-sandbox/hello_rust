pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[test]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    #[test]
    fn less_than_1() {
        Guess::new(0);
    }
}
