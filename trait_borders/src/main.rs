use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Наибольший член x равен {}", self.x);
        } else {
            println!("Наибольший член y равен {}", self.y);
        }
    }
}

fn main() {

}
