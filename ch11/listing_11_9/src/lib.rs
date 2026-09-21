pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Значение догадки должно быть больше или равно 1, получено {}.", value);
        } else if value > 100 {
            panic!("Значение догадки должно быть меньше или равно 100, получено {}.", value);
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Значение догадки должно быть меньше или равно 100")]
    fn greather_then_100() {
        Guess::new(200);
    }
}
