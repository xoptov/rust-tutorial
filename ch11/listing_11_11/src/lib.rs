pub fn add(left: i32) -> i32 {
    left + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add(3));
    }

    #[test]
    fn one_handred() {
        assert_eq!(102, add(100));
    }
}
