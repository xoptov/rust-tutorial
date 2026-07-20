mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    pub enum Apetizer {
        Soup,
        Saled
    }
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("персики")
            }
        } 
    }
    fn fix_incorrent_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
