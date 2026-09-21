pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Значение догадки должно быть между 1 и 100, получено {}.", value);
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greather_then_100() {
        Guess::new(200);
    }
}
